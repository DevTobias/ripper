use anyhow::Context;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use futures::stream::SplitSink;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::{path::Path, thread};
use tokio::fs;
use tracing::error;

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

#[derive(Deserialize, Clone, Debug)]
pub struct RipPayload {
    device: String,
    titles: Vec<usize>,
    profile: String,
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
pub async fn rip_movie_websocket_handler(Query(params): Query<RipPayload>, State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    async fn handle_makemkv_rip(
        sender: &mut SplitSink<WebSocket, Message>, cancel_flag: Arc<AtomicBool>, command: String, makemkv_mutex: Arc<std::sync::Mutex<()>>, output_dir: String,
        device: String, titles: Vec<usize>,
    ) {
        let (rip_sender, rip_receiver) = mpsc::channel::<(&str, Option<ProgressPayload>)>();

        thread::spawn(move || {
            if let Err(e) = rip_titles(&command, &makemkv_mutex, cancel_flag, rip_sender, &output_dir, &device, &titles) {
                error!("failed to rip titles: {:?}", e);
            }
        });

        while let Ok((event_type, payload)) = rip_receiver.recv() {
            let message = match event_type {
                "progress" => {
                    if let Some(payload) = payload {
                        format!(
                            r#"{{ "type": "ripping_progress", "payload": {{ "stepTitle": "{}", "stepDetails": "{}", "progress": {}, "step": {}, "eta": {} }} }}"#,
                            payload.step_title, payload.step_details, payload.progress, payload.step, payload.eta
                        )
                    } else {
                        continue;
                    }
                }
                "done" => r#"{"type": "ripping_done"}"#.to_string(),
                _ => continue,
            };

            if let Err(e) = sender.send(Message::Text(message)).await {
                error!("Failed to send WebSocket message: {:?}", e);
                break;
            }
        }
    }

    async fn handle_handbrake_encoding(
        sender: &mut SplitSink<WebSocket, Message>, cancel_flag: Arc<AtomicBool>, command: String, output_dir: String, files: Vec<String>, profile: Profile,
    ) {
        let (encoding_sender, encoding_receiver) = mpsc::channel::<(&str, Option<EncodingProgressPayload>)>();

        thread::spawn(move || {
            let files = files.iter().map(|f| f.as_str()).collect::<Vec<&str>>();

            if let Err(e) = encode_files(&command, &profile, &files, &output_dir, cancel_flag, encoding_sender) {
                error!("failed to encode titles: {:?}", e);
            }
        });

        while let Ok((event_type, payload)) = encoding_receiver.recv() {
            let message = match event_type {
                "progress" => {
                    if let Some(payload) = payload {
                        format!(
                            r#"{{ "type": "encoding_progress", "payload": {{ "stepTitle": "{}", "stepDetails": "{}", "progress": {}, "step": {}, "eta": {} }} }}"#,
                            "Encoding", "Encoding", payload.progress, payload.step, payload.eta
                        )
                    } else {
                        continue;
                    }
                }
                "done" => r#"{"type": "encoding_done"}"#.to_string(),
                _ => continue,
            };

            if let Err(e) = sender.send(Message::Text(message)).await {
                error!("Failed to send WebSocket message: {:?}", e);
                break;
            }
        }
    }

    async fn handle_rip_socket(socket: WebSocket, state: AppState, params: RipPayload) {
        let (mut socket_sender, mut socket_receiver) = socket.split();
        let cancel_flag = Arc::new(AtomicBool::new(false));

        let profiles = get_encoding_profiles(&state.encoding_profiles_path).unwrap();
        let disc = read_disc_properties(&state.makemkv_command, &params.device, &state.makemkv_mutex)
            .context("failed to read disc properties")
            .unwrap();

        let rip_cancel_flag = cancel_flag.clone();
        let rip_command = state.makemkv_command.clone();
        let rip_output_dir = state.output_dir.clone();
        let rip_device = params.device.clone();
        let rip_tiles = params.titles.clone();
        let rip_makemkv_mutex = state.makemkv_mutex.clone();

        let encode_cancel_flag = cancel_flag.clone();
        let encode_command = state.handbrake_command.clone();
        let encode_output_dir = state.output_dir.clone();
        let encode_profile = profiles.iter().find(|p| p.id == params.profile).unwrap().clone();

        let upload_cancel_flag = cancel_flag.clone();

        let ripped_titles = params
            .titles
            .iter()
            .map(|title| disc.titles.iter().find(|t| t.id == *title).unwrap().to_owned())
            .collect::<Vec<Title>>();

        let encode_titles = ripped_titles.clone();

        tokio::spawn(async move {
            while let Some(msg) = socket_receiver.next().await {
                if let Ok(Message::Text(text)) = msg {
                    if text.trim() == "cancel" {
                        cancel_flag.store(true, Ordering::Relaxed);

                        for title in &ripped_titles {
                            let rip_file = Path::new(&state.output_dir).join(&title.output_file_name);
                            let encoded_file = Path::new(&state.output_dir).join("/encoding/").join(&title.output_file_name);

                            if rip_file.exists() {
                                fs::remove_file(rip_file).await.unwrap();
                            }

                            if encoded_file.exists() {
                                fs::remove_file(encoded_file).await.unwrap();
                            }
                        }
                    }
                }
            }
        });

        let ripped_files = encode_titles
            .iter()
            .map(|title| {
                let file = Path::new(&rip_output_dir).join(&title.output_file_name);
                file.to_str().unwrap().to_string()
            })
            .collect::<Vec<String>>();

        handle_makemkv_rip(&mut socket_sender, rip_cancel_flag, rip_command, rip_makemkv_mutex, rip_output_dir, rip_device, rip_tiles).await;

        if encode_cancel_flag.load(Ordering::Relaxed) {
            return;
        }

        handle_handbrake_encoding(&mut socket_sender, encode_cancel_flag, encode_command, encode_output_dir, ripped_files, encode_profile).await;

        if upload_cancel_flag.load(Ordering::Relaxed) {
            return;
        }

        // let encoded_files = encode_titles
        //     .iter()
        //     .map(|title| {
        //         let file = Path::new(&rip_output_dir).join(&title.output_file_name);
        //         file.to_str().unwrap().to_string()
        //     })
        //     .collect::<Vec<String>>();

        //
    }

    ws.on_upgrade(move |socket| handle_rip_socket(socket, state, params))
}
