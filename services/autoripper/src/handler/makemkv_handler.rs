use axum::extract::ws::{Message, WebSocket};
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::{path::Path, thread};
use tokio::fs;
use tracing::error;

use makemkv_core::{detect_devices, filter_movie_main_features, filter_tv_series_main_features, read_disc_properties};
use makemkv_core::{rip_titles, ProgressPayload};

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

#[derive(Deserialize, Debug)]
pub struct RipMoviePayload {
    device: String,
    titles: Vec<usize>,
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
    match detect_devices(&state.command, &state.makemkv_mutex) {
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
    match read_disc_properties(&state.command, &params.device, &state.makemkv_mutex) {
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
    match read_disc_properties(&state.command, &params.device, &state.makemkv_mutex) {
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
pub async fn rip_movie_websocket_handler(Query(params): Query<RipMoviePayload>, State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    async fn handle_rip_socket(socket: WebSocket, state: AppState, params: RipMoviePayload) {
        let (mut socket_sender, mut socket_receiver) = socket.split();
        let (sender, receiver) = mpsc::channel::<(&str, Option<ProgressPayload>)>();

        let cancel_flag = Arc::new(AtomicBool::new(false));

        let borrowed_cancel_flag = cancel_flag.clone();
        let borrowed_titles = params.titles.clone();
        let borrowed_output_dir = state.output_dir.clone();

        let disc = match read_disc_properties(&state.command, &params.device, &state.makemkv_mutex) {
            Ok(disc) => disc,
            Err(e) => {
                error!("failed to read disc properties: {:?}", e);
                let _ = socket_sender
                    .send(Message::Text(r#"{"type": "error", "message": "failed to read disc properties"}"#.to_string()))
                    .await;
                return;
            }
        };

        tokio::spawn(async move {
            while let Some(msg) = socket_receiver.next().await {
                if let Ok(Message::Text(text)) = msg {
                    if text.trim() == "cancel" {
                        borrowed_cancel_flag.store(true, Ordering::Relaxed);

                        for main_feature in &params.titles {
                            if let Some(title) = disc.titles.iter().find(|title| title.id == *main_feature) {
                                let file_name = &title.output_file_name;
                                if let Err(e) = fs::remove_file(Path::new(&state.output_dir).join(file_name)).await {
                                    error!("failed to remove file {}: {:?}", file_name, e);
                                }
                            }
                        }
                    }
                }
            }
        });

        thread::spawn(move || {
            if let Err(e) = rip_titles(&state.command, &state.makemkv_mutex, cancel_flag, sender, &borrowed_output_dir, &params.device, &borrowed_titles) {
                error!("failed to rip titles: {:?}", e);
            }
        });

        while let Ok((event_type, payload)) = receiver.recv() {
            let message = match event_type {
                "progress" => {
                    if let Some(payload) = payload {
                        format!(
                            r#"{{ "type": "progress", "payload": {{ "stepTitle": "{}", "stepDetails": "{}", "progress": {}, "step": {} }} }}"#,
                            payload.step_title, payload.step_details, payload.progress, payload.step
                        )
                    } else {
                        continue;
                    }
                }
                "done" => r#"{"type": "done"}"#.to_string(),
                _ => continue,
            };

            if let Err(e) = socket_sender.send(Message::Text(message)).await {
                error!("Failed to send WebSocket message: {:?}", e);
                break;
            }
        }
    }

    ws.on_upgrade(move |socket| handle_rip_socket(socket, state, params))
}
