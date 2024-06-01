use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use handbrake_core::get_encoding_profiles;
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

#[derive(Deserialize)]
pub struct MediaDetailsPayload {
    id: u32,
}

pub async fn get_tv_details(State(state): State<AppState>, Json(payload): Json<MediaDetailsPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.get_tv_series(payload.id).await.unwrap()))
}

pub async fn get_movie_details(State(state): State<AppState>, Json(payload): Json<MediaDetailsPayload>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.tmdb_client.get_movie(payload.id).await.unwrap()))
}

pub async fn get_devices_handler(State(state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, Json(detect_devices(&state.command).unwrap()))
}

pub async fn get_encoding_profiles_handler() -> impl IntoResponse {
    (StatusCode::OK, Json(get_encoding_profiles()))
}
