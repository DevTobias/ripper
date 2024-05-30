use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use makemkv_core::detect_devices;
use serde::Deserialize;

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

pub async fn get_devices_handler(State(state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, Json(detect_devices(&state.command).unwrap()))
}
