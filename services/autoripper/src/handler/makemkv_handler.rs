use anyhow::Context;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use futures::stream::{SplitSink, SplitStream};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::{path::Path, thread};
use tokio::fs;
use tokio::sync::Mutex;
use tracing::{error, info};
use utils::{upload_file_with_sftp, UploadProgressPayload};

use handbrake_core::{encode_files, get_encoding_profiles, EncodingProgressPayload, Profile};
use makemkv_core::ProgressPayload;
use makemkv_core::{detect_devices, filter_movie_main_features, filter_tv_series_main_features, read_disc_properties, rip_titles, Title};

use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct MovieTitlesPayload {
    langs: Vec<String>,
    tmdb_id: u32,
    device: String,
}

#[derive(Deserialize, Debug)]
pub struct TvShowTitlesPayload {
    langs: Vec<String>,
    tmdb_id: u32,
    device: String,
    season: u16,
    episodes: Vec<u32>,
}

/// Handles requests to retrieve a list of devices.
///
/// This handler fetches the available devices using the `detect_devices`
/// function from the `makemkv_core` module and returns them as a JSON response.
///
/// # Arguments
///
/// * `state` - The application state containing the command and MakeMKV mutex.
///
/// # Returns
///
/// A JSON response containing the list of detected devices or an error response if detection fails.
///
/// # Errors
///
/// Returns an `AppError` if the device detection fails.
/// ```
pub async fn get_devices_handler(State(state): State<AppState>) -> impl IntoResponse {
    match detect_devices(&state.makemkv_command, &state.makemkv_mutex) {
        Ok(devices) => (StatusCode::OK, Json(devices)).into_response(),
        Err(err) => {
            error!("Failed to detect devices: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "failed to detect devices" }))).into_response()
        }
    }
}

/// Handles requests to retrieve disc titles and filter them based on the specified parameters.
///
/// This handler reads the disc properties and applies filters based on the provided
/// `disc_type` and `langs`. It then returns the filtered disc titles as a JSON response.
///
/// # Arguments
///
/// * `state` - The application state containing the necessary dependencies.
/// * `params` - The query parameters containing the device, disc type, TMDB ID, and languages.
///
/// # Returns
///
/// A JSON response containing the filtered disc titles or an error response if the operation fails.
///
/// # Errors
///
/// Returns an `AppError` if reading disc properties or filtering the titles fails.
/// ```
pub async fn get_movie_titles_handler(State(state): State<AppState>, Query(params): Query<MovieTitlesPayload>) -> impl IntoResponse {
    match read_disc_properties(&state.makemkv_command, &params.device, &state.makemkv_mutex) {
        Ok(disc) => {
            let langs: Vec<&str> = params.langs.iter().map(|lang| lang.as_str()).collect();

            match filter_movie_main_features(disc, &langs, params.tmdb_id, &state.tmdb_client).await {
                Ok(filtered_disc) => (StatusCode::OK, Json(filtered_disc)).into_response(),
                Err(err) => {
                    error!("failed to filter movie main features: {}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "ailed to filter movie main features" }))).into_response()
                }
            }
        }
        Err(err) => {
            error!("failed to read disc properties: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "failed to read disc properties" }))).into_response()
        }
    }
}

/// Handles requests to retrieve TV show titles and filter them based on the specified parameters.
///
/// This handler reads the disc properties and applies filters based on the provided
/// `langs`, `season`, and `episodes`. It then returns the filtered TV show titles as a JSON response.
///
/// # Arguments
///
/// * `state` - The application state containing the necessary dependencies.
/// * `params` - The query parameters containing the device, TMDB ID, season, episodes, and languages.
///
/// # Returns
///
/// A JSON response containing the filtered TV show titles or an error response if the operation fails.
///
/// # Errors
///
/// Returns an `AppError` if reading disc properties or filtering the titles fails.
/// ```
pub async fn get_tv_show_titles_handler(State(state): State<AppState>, Query(params): Query<TvShowTitlesPayload>) -> impl IntoResponse {
    match read_disc_properties(&state.makemkv_command, &params.device, &state.makemkv_mutex) {
        Ok(disc) => {
            let langs: Vec<&str> = params.langs.iter().map(|lang| lang.as_str()).collect();
            let episodes: Vec<u16> = params.episodes.iter().map(|&e| e as u16).collect();

            match filter_tv_series_main_features(disc, &langs, params.season, &episodes, params.tmdb_id, &state.tmdb_client).await {
                Ok(filtered_disc) => (StatusCode::OK, Json(filtered_disc)).into_response(),
                Err(err) => {
                    error!("failed to filter tv show main features: {}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "ailed to filter tv show main features" }))).into_response()
                }
            }
        }
        Err(err) => {
            error!("failed to read disc properties: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "failed to read disc properties" }))).into_response()
        }
    }
}

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
}

struct RippingHandler {
    cancel_flag: Arc<AtomicBool>,
    state: AppState,
    params: RipPayload,
    titles: Vec<Title>,
    profiles: Vec<Profile>,
}

impl RippingHandler {
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
        let quality_profile_id = self.params.quality_profile.clone();
        let root_folder = self.params.root_folder.clone();
        let remote_host = self.state.remote_host.clone();
        let remote_user = self.state.remote_user.clone();
        let remote_password = self.state.remote_password.clone();

        let upload_handle = tokio::spawn(async move {
            if media_type == "movie" {
                let metadata = serde_json::from_str::<RipMovieMetadata>(&metadata).unwrap();

                for (i, file) in files.iter().enumerate() {
                    info!("Uploading movie: {}", file);

                    let movie = radarr_client.create_movie(metadata.tmdb_id, quality_profile_id, &root_folder).await.unwrap();

                    let local_file_name = Path::new(&file).file_name().unwrap().to_string_lossy().to_string();
                    let remote_path = Path::new(&movie.path).join(format!("[Bluray-1080p]_{}", local_file_name));

                    if let Err(e) =
                        upload_file_with_sftp(&file, remote_path.to_str().unwrap(), i as u32, &remote_host, &remote_user, &remote_password, &cancel_flag, &upload_sender)
                    {
                        error!("failed to upload file: {:?}", e);
                    }

                    radarr_client.scan_rename_movie(movie.id).await.unwrap();
                }
            }
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
