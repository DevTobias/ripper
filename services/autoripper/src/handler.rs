use std::net::SocketAddr;

use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::TypedHeader;
use serde::Deserialize;

use handbrake_core::get_encoding_profiles;
use makemkv_core::detect_devices;

use crate::AppState;

#[derive(Deserialize)]
pub struct SearchPayload {
    query: String,
    lang: String,
}

pub async fn search_movie_handler(State(state): State<AppState>, Json(payload): Json<SearchPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.search_movies(&payload.query, &payload.lang).await.unwrap()))
}

pub async fn search_tv_series_handler(State(state): State<AppState>, Json(payload): Json<SearchPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.search_tv_series(&payload.query, &payload.lang).await.unwrap()))
}

#[derive(Deserialize)]
pub struct MediaDetailsPayload {
    id: u32,
}

pub async fn get_tv_details_handler(State(state): State<AppState>, Json(payload): Json<MediaDetailsPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.get_tv_series(payload.id).await.unwrap()))
}

pub async fn get_movie_details_handler(State(state): State<AppState>, Json(payload): Json<MediaDetailsPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.get_movie(payload.id).await.unwrap()))
}

pub async fn get_devices_handler(State(state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, Json(detect_devices(&state.command).unwrap()))
}

pub async fn get_encoding_profiles_handler() -> impl IntoResponse {
    (StatusCode::OK, Json(get_encoding_profiles()))
}

pub async fn rip_websocket_handler(ws: WebSocketUpgrade, user_agent: Option<TypedHeader<headers::UserAgent>>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    println!("`{user_agent}` at {addr} connected.");

    async fn handle_rip_socket(mut socket: WebSocket, who: SocketAddr) {
        for i in 1..5 {
            if socket.send(Message::Text(format!("Hi {i} times!"))).await.is_err() {
                println!("client {who} abruptly disconnected");
                return;
            }
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
    }

    ws.on_upgrade(move |socket| handle_rip_socket(socket, addr))
}
