use anyhow::Context;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::response::IntoResponse;
use axum_extra::extract::Query;
use futures::stream::{SplitSink, SplitStream};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::{path::Path, thread};
use tokio::fs;
use tokio::sync::Mutex;
use tracing::{error, info};
use utils::{upload_file_with_sftp, UploadProgressPayload};

use handbrake_core::{encode_files, get_encoding_profiles, EncodingProgressPayload, Profile};
use makemkv_core::ProgressPayload;
use makemkv_core::{read_disc_properties, rip_titles, Title};

use crate::AppState;

#[derive(Deserialize, Clone, Debug)]
pub struct RipPayload {
    device: String,
    titles: Vec<usize>,
    encoding_profile: String,
    quality_profile: u32,
    root_folder: String,
    media_type: String,
    metadata: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RipMovieMetadata {
    pub tmdb_id: u32,
    pub title: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RipTvShowMetadata {
    pub tvdb_id: u32,
    pub title: String,
    pub series_type: String,
    pub season: u32,
    pub episodes: Vec<u32>,
}

struct RippingHandler {
    cancel_flag: Arc<AtomicBool>,
    state: AppState,
    params: RipPayload,
    titles: Vec<Title>,
    profiles: Vec<Profile>,
}

impl RippingHandler {
    /// Creates a new instance of `RippingHandler`.
    ///
    /// This function initializes the `RippingHandler` by reading encoding profiles,
    /// disc properties, and selecting the titles to be processed.
    ///
    /// # Arguments
    ///
    /// * `state` - The application state containing various configurations and clients.
    /// * `params` - Parameters for ripping, including device and titles to be processed.
    ///
    /// # Returns
    ///
    /// A new instance of `RippingHandler`.
    pub fn new(state: AppState, params: RipPayload) -> Self {
        let profiles = get_encoding_profiles(&state.encoding_profiles_path).unwrap();

        let disc = read_disc_properties(&state.makemkv_command, &params.device, &state.makemkv_mutex)
            .context("failed to read disc properties")
            .unwrap();

        let titles = params
            .titles
            .iter()
            .map(|title| disc.titles.iter().find(|t| t.id == *title).unwrap().to_owned())
            .collect::<Vec<Title>>();

        Self { state, params, titles, profiles, cancel_flag: Arc::new(AtomicBool::new(false)) }
    }

    /// Handles cancellation of the ripping process via WebSocket.
    ///
    /// This function listens for "cancel" messages over the WebSocket connection and
    /// sets the cancellation flag. It also removes any partially processed files upon
    /// cancellation.
    ///
    /// # Arguments
    ///
    /// * `socket_receiver` - A stream of incoming WebSocket messages.
    pub async fn handle_cancellation(&self, mut socket_receiver: SplitStream<WebSocket>) {
        let cancel_flag = self.cancel_flag.clone();
        let titles = self.titles.clone();
        let output_dir = self.state.output_dir.clone();

        tokio::spawn(async move {
            while let Some(msg) = socket_receiver.next().await {
                if let Ok(Message::Text(text)) = msg {
                    if text.trim() == "cancel" {
                        cancel_flag.store(true, Ordering::Relaxed);

                        for title in &titles {
                            let rip_file = Path::new(&output_dir).join(&title.output_file_name);

                            if rip_file.exists() {
                                fs::remove_file(rip_file).await.unwrap();
                            }

                            let encoded_file = Path::new(&output_dir).join("/encoding/").join(&title.output_file_name);

                            if encoded_file.exists() {
                                fs::remove_file(encoded_file).await.unwrap();
                            }
                        }
                    }
                }
            }
        });
    }

    /// Rips the selected titles from the disc.
    ///
    /// This function spawns a new thread to handle the ripping process and sends
    /// progress updates over the WebSocket connection.
    ///
    /// # Arguments
    ///
    /// * `socket_sender` - A mutable reference to the WebSocket sender for sending messages.
    pub async fn rip_titles(&self, socket_sender: &mut SplitSink<WebSocket, Message>) {
        let (rip_sender, rip_receiver) = mpsc::channel::<(&str, Option<ProgressPayload>)>();

        let command = self.state.makemkv_command.clone();
        let makemkv_mutex = self.state.makemkv_mutex.clone();
        let output_dir = self.state.output_dir.clone();
        let device = self.params.device.clone();
        let titles = self.params.titles.clone();
        let cancel_flag = self.cancel_flag.clone();

        thread::spawn(move || {
            if let Err(e) = rip_titles(&command, &makemkv_mutex, cancel_flag, rip_sender, &output_dir, &device, &titles) {
                error!("failed to rip titles: {:?}", e);
            }
        });

        while let Ok((event_type, payload)) = rip_receiver.recv() {
            if payload.is_none() {
                continue;
            }

            let payload = payload.unwrap();

            let message = match event_type {
                "progress" => format!(
                    r#"{{ "type": "ripping_progress", "payload": {{ "label": "{}", "progress": {}, "step": {}, "eta": {} }} }}"#,
                    payload.step_details, payload.progress, payload.step, payload.eta
                ),
                "done" => r#"{"type": "ripping_done"}"#.to_string(),
                _ => continue,
            };

            if let Err(e) = socket_sender.send(Message::Text(message)).await {
                error!("Failed to send WebSocket message: {:?}", e);
                break;
            }
        }
    }

    /// Encodes the ripped files using the specified encoding profile.
    ///
    /// This function spawns a new thread to handle the encoding process and sends
    /// progress updates over the WebSocket connection.
    ///
    /// # Arguments
    ///
    /// * `socket_sender` - A mutable reference to the WebSocket sender for sending messages.
    pub async fn encode_files(&self, socket_sender: &mut SplitSink<WebSocket, Message>) {
        let (encoding_sender, encoding_receiver) = mpsc::channel::<(&str, Option<EncodingProgressPayload>)>();

        let cancel_flag = self.cancel_flag.clone();
        let command = self.state.handbrake_command.clone();
        let output_dir = self.state.output_dir.clone();
        let profile = self.profiles.iter().find(|p| p.id == self.params.encoding_profile).unwrap().clone();

        let files: Vec<String> = self
            .titles
            .iter()
            .map(|title| Path::new(&self.state.output_dir).join(&title.output_file_name).to_string_lossy().to_string())
            .collect();

        thread::spawn(move || {
            let files = files.iter().map(|f| f.as_str()).collect::<Vec<&str>>();

            if let Err(e) = encode_files(&command, &profile, &files, &output_dir, cancel_flag, encoding_sender) {
                error!("failed to encode titles: {:?}", e);
            }
        });

        while let Ok((event_type, payload)) = encoding_receiver.recv() {
            if payload.is_none() {
                continue;
            }

            let payload = payload.unwrap();

            let message = match event_type {
                "progress" => format!(
                    r#"{{ "type": "encoding_progress", "payload": {{ "label": "{}", "progress": {}, "step": {}, "eta": {} }} }}"#,
                    "Encoding", payload.progress, payload.step, payload.eta
                ),
                "done" => r#"{"type": "encoding_done"}"#.to_string(),
                _ => continue,
            };

            if let Err(e) = socket_sender.send(Message::Text(message)).await {
                error!("Failed to send WebSocket message: {:?}", e);
                break;
            }
        }
    }

    /// Uploads the encoded files to the specified remote server.
    ///
    /// This function spawns a new task to handle the file upload process and sends
    /// progress updates over the WebSocket connection.
    ///
    /// # Arguments
    ///
    /// * `socket_sender` - A shared and mutable reference to the WebSocket sender for sending messages.
    pub async fn upload_files(&self, socket_sender: Arc<Mutex<SplitSink<WebSocket, Message>>>) {
        let (upload_sender, upload_receiver) = mpsc::channel::<(&str, Option<UploadProgressPayload>)>();

        let files: Vec<String> = self
            .titles
            .iter()
            .map(|title| {
                Path::new(&self.state.output_dir)
                    .join("encoding/")
                    .join(&title.output_file_name)
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        info!("Uploading Files: {:?}", files);

        let cancel_flag = self.cancel_flag.clone();
        let metadata = self.params.metadata.clone();
        let media_type = self.params.media_type.clone();
        let radarr_client = self.state.radarr_client.clone();
        let sonarr_client = self.state.sonarr_client.clone();
        let jellyfin_client = self.state.jellyfin_client.clone();
        let quality_profile_id = self.params.quality_profile.clone();
        let root_folder = self.params.root_folder.clone();
        let remote_host = self.state.remote_host.clone();
        let remote_user = self.state.remote_user.clone();
        let remote_password = self.state.remote_password.clone();

        let upload_handle = tokio::spawn(async move {
            if media_type == "movie" {
                let metadata = serde_json::from_str::<RipMovieMetadata>(&metadata).unwrap();

                let file = files.first().unwrap();

                info!("Uploading movie: {}", file);

                let movie = radarr_client
                    .create_movie(metadata.tmdb_id, &metadata.title, quality_profile_id, &root_folder)
                    .await
                    .unwrap();

                let local_file_name = Path::new(&file).file_name().unwrap().to_string_lossy().to_string();
                let remote_path = Path::new(&movie.path).join(format!("[Bluray-1080p]_{}", local_file_name));

                if let Err(e) = upload_file_with_sftp(&file, remote_path.to_str().unwrap(), 0, &remote_host, &remote_user, &remote_password, &cancel_flag, &upload_sender)
                {
                    error!("failed to upload file: {:?}", e);
                }

                radarr_client.scan_rename_movie(movie.id).await.ok();
            }

            if media_type == "tv_show" {
                let metadata = serde_json::from_str::<RipTvShowMetadata>(&metadata).unwrap();

                let tv_show = sonarr_client
                    .create_tv_show(metadata.tvdb_id, &metadata.title, &metadata.series_type, quality_profile_id, &root_folder)
                    .await
                    .unwrap();

                for (i, file) in files.iter().enumerate() {
                    info!("Uploading TV show: {}", file);

                    let season_path = Path::new(&tv_show.path).join(format!("Season {:0>2}", metadata.season));
                    let file_name = Path::new(&file).file_name().unwrap().to_string_lossy().to_string();
                    let prefixed_file_name = format!("[Bluray-1080p]_S{:0>2}E{:0>2}_{}", metadata.season, metadata.episodes[i], file_name);
                    let remote_path = season_path.join(prefixed_file_name);

                    if let Err(e) =
                        upload_file_with_sftp(&file, remote_path.to_str().unwrap(), i as u32, &remote_host, &remote_user, &remote_password, &cancel_flag, &upload_sender)
                    {
                        error!("failed to upload file: {:?}", e);
                    }
                }

                sonarr_client.scan_rename_tv_show(tv_show.id).await.ok();
            }

            jellyfin_client.library_scan().await.ok();
            upload_sender.send(("done", None)).unwrap();
        });

        let receiver_handle = tokio::spawn(async move {
            while let Ok((event_type, payload)) = upload_receiver.recv() {
                if payload.is_none() {
                    continue;
                }

                let payload = payload.unwrap();

                let message = match event_type {
                    "progress" => format!(
                        r#"{{ "type": "upload_progress", "payload": {{ "label": "{}", "progress": {}, "step": {}, "eta": {} }} }}"#,
                        "Uploading", payload.progress, payload.step, payload.eta
                    ),
                    "done" => r#"{"type": "uploading_done"}"#.to_string(),
                    _ => continue,
                };

                let mut socket_sender_guard = socket_sender.lock().await;
                if let Err(e) = socket_sender_guard.send(Message::Text(message)).await {
                    error!("Failed to send WebSocket message: {:?}", e);
                    break;
                }
            }
        });

        if let Err(e) = upload_handle.await {
            error!("Upload task failed: {:?}", e);
        }

        if let Err(e) = receiver_handle.await {
            error!("Receiver task failed: {:?}", e);
        }
    }

    /// Handles the entire ripping process from ripping to uploading files.
    ///
    /// This function coordinates the cancellation, ripping, encoding, and uploading
    /// processes, and sends updates over the WebSocket connection.
    ///
    /// # Arguments
    ///
    /// * `socket` - The WebSocket connection for sending and receiving messages.
    pub async fn handle(self, socket: WebSocket) {
        let (mut socket_sender, socket_receiver) = socket.split();

        self.handle_cancellation(socket_receiver).await;

        self.rip_titles(&mut socket_sender).await;
        if self.cancel_flag.load(Ordering::Relaxed) {
            return;
        }

        self.encode_files(&mut socket_sender).await;
        if self.cancel_flag.load(Ordering::Relaxed) {
            return;
        }

        let mutex_socket_sender: Arc<Mutex<SplitSink<WebSocket, Message>>> = Arc::new(Mutex::new(socket_sender));
        self.upload_files(mutex_socket_sender).await;
    }
}

/// Handles WebSocket connections to rip Blu-ray discs using MakeMKV and stream progress updates.
///
/// This handler accepts a WebSocket connection, receives ripping parameters, and initiates
/// the ripping process. It streams progress updates to the WebSocket client and supports
/// cancellation of the ripping process upon client request.
///
/// # Arguments
///
/// * `params` - The parameters for the ripping process, including device and title information.
/// * `state` - The shared application state, containing configuration and shared resources.
/// * `ws` - The WebSocket upgrade request.
///
/// # Returns
///
/// A response that upgrades the connection to a WebSocket connection.
pub async fn rip_websocket_handler(Query(params): Query<RipPayload>, State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    info!("RipPayload: {:?}", params);
    ws.on_upgrade(move |socket| async move {
        RippingHandler::new(state, params).handle(socket).await;
    })
}
