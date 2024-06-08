use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use serde::Deserialize;
use serde_json::json;

use crate::AppState;

#[derive(Deserialize)]
pub struct MediaTypePayload {
    media_type: String,
}

pub async fn get_quality_profile_handler(State(state): State<AppState>, Query(params): Query<MediaTypePayload>) -> impl IntoResponse {
    if params.media_type == "movie" {
        let profiles = state.radarr_client.client.get_quality_profiles().await.unwrap();
        return (StatusCode::OK, Json(profiles)).into_response();
    }

    (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid media type" }))).into_response()
}

pub async fn get_root_folder_handler(State(state): State<AppState>, Query(params): Query<MediaTypePayload>) -> impl IntoResponse {
    if params.media_type == "movie" {
        let profiles = state.radarr_client.client.get_root_folders().await.unwrap();
        return (StatusCode::OK, Json(profiles)).into_response();
    }

    (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid media type" }))).into_response()
}
