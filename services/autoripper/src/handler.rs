use anyhow::Result;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::Query;
use futures::{sink::SinkExt, stream::StreamExt};

use serde::Deserialize;
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
};
use tokio::fs;

use handbrake_core::get_encoding_profiles;
use makemkv_core::{detect_devices, filter_movie_candidates, read_properties, rip_titles};
use tracing::info;

use crate::{error::AppError, AppState};

#[derive(Deserialize)]
pub struct SearchPayload {
    query: String,
    lang: String,
}

#[derive(Deserialize)]
pub struct MediaDetailsPayload {
    id: u32,
}

#[derive(Deserialize, Debug)]
pub struct RipMoviePayload {
    #[serde(default)]
    langs: Vec<String>,
    tmdb_id: u32,
    device: String,
}

#[derive(Debug)]
pub struct ProgressPayload {
    step_title: String,
    step_details: String,
    progress: f32,
    step: usize,
}

//

pub async fn search_movie_handler(State(state): State<AppState>, Json(payload): Json<SearchPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.search_movies(&payload.query, &payload.lang).await.unwrap()))
}

pub async fn search_tv_series_handler(State(state): State<AppState>, Json(payload): Json<SearchPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.search_tv_series(&payload.query, &payload.lang).await.unwrap()))
}

pub async fn get_tv_details_handler(State(state): State<AppState>, Json(payload): Json<MediaDetailsPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.get_tv_series(payload.id).await.unwrap()))
}

pub async fn get_movie_details_handler(State(state): State<AppState>, Json(payload): Json<MediaDetailsPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.get_movie(payload.id).await.unwrap()))
}

pub async fn get_encoding_profiles_handler() -> impl IntoResponse {
    (StatusCode::OK, Json(get_encoding_profiles()))
}

//

pub async fn get_devices_handler(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let devices = detect_devices(&state.command, &state.makemkv_mutex)?;
    Ok(Json(devices).into_response())
}

pub async fn rip_movie_websocket_handler(Query(params): Query<RipMoviePayload>, State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    async fn handle_rip_socket(socket: WebSocket, state: AppState, params: RipMoviePayload) {
        let mut disc = read_properties(&state.command, &params.device, state.makemkv_mutex).unwrap();

        disc = filter_movie_candidates(disc, params.langs.iter().map(|lang| lang.as_str()).collect(), params.tmdb_id, &state.tmdb_client)
            .await
            .unwrap();

        let main_feature = disc.titles[0].clone();
        info!("detected main feature, {:?}", main_feature);

        let (mut socket_sender, mut socket_receiver) = socket.split();

        let (sender, receiver) = mpsc::channel::<(&str, Option<ProgressPayload>)>();

        let cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag_ws = cancel_flag.clone();

        let output_dir = state.output_dir.clone();

        tokio::spawn(async move {
            while let Some(msg) = socket_receiver.next().await {
                if let Ok(Message::Text(text)) = msg {
                    if text.trim() == "cancel" {
                        cancel_flag_ws.store(true, Ordering::Relaxed);
                        fs::remove_file(Path::new(&state.output_dir).join(&main_feature.output_file_name)).await.unwrap();
                    }
                }
            }
        });

        thread::spawn(move || {
            let _ = rip_titles(&state.command, &params.device, vec![main_feature.id], &output_dir, cancel_flag, &|step_title: String, step_details: String, progress: f32, step: usize| {
                let payload = ProgressPayload { step_title, step_details, progress, step };

                sender.send(("progress", Some(payload))).unwrap();
            });

            sender.send(("done", None)).unwrap();
        });

        while let Ok((event_type, payload)) = receiver.recv() {
            if event_type == "progress" {
                let payload = payload.unwrap();

                let message = format!(
                    r#"{{ "type": "progress", "payload": {{ "stepTitle" : "{}", "stepDetails": "{}", "progress": {}, "step": {} }} }}"#,
                    payload.step_title, payload.step_details, payload.progress, payload.step
                );

                socket_sender.send(Message::Text(message.to_string())).await.unwrap();
            }

            if event_type == "done" {
                let message = r#"{{ "type": "done" }}"#;
                socket_sender.send(Message::Text(message.to_string())).await.unwrap();
            }
        }
    }

    ws.on_upgrade(move |socket| handle_rip_socket(socket, state, params))
}
