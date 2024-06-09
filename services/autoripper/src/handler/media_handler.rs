use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use serde::Deserialize;
use serde_json::json;

use handbrake_core::get_encoding_profiles;

use crate::AppState;

#[derive(Deserialize)]
pub struct MediaTypePayload {
    media_type: String,
}

/// Handles requests to retrieve encoding profiles.
///
/// This handler fetches the available encoding profiles using the `get_encoding_profiles`
/// function from the `handbrake_core` module and returns them as a JSON response.
///
/// # Returns
///
/// A JSON response containing the list of encoding profiles.
///
/// # Errors
///
/// This function does not currently handle any errors as the `get_encoding_profiles`
/// function is expected to return valid data or panic.
pub async fn get_encoding_profiles_handler(State(state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, Json(get_encoding_profiles(&state.encoding_profiles_path).unwrap()))
}

/// Handles the request to get quality profiles for a specified media type.
///
/// Depending on the media type provided in the query parameters, this function
/// will fetch quality profiles from either Radarr (for movies) or Sonarr (for TV shows).
///
/// # Arguments
///
/// * `State(state)`: The application state containing clients for Radarr and Sonarr.
/// * `Query(params)`: The query parameters containing the media type.
///
/// # Returns
///
/// An HTTP response with the quality profiles in JSON format for the specified media type,
/// or a bad request response if the media type is invalid.
///
/// # Errors
///
/// This function will return a `BAD_REQUEST` status code if the media type is not "movie" or "tv_show".
/// ```
pub async fn get_quality_profile_handler(State(state): State<AppState>, Query(params): Query<MediaTypePayload>) -> impl IntoResponse {
    match params.media_type.as_str() {
        "movie" => (StatusCode::OK, Json(state.radarr_client.client.get_quality_profiles().await.unwrap())).into_response(),
        "tv_show" => (StatusCode::OK, Json(state.sonarr_client.client.get_quality_profiles().await.unwrap())).into_response(),
        _ => (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid media type" }))).into_response(),
    }
}

/// Handles the request to get root folders for a specified media type.
///
/// Depending on the media type provided in the query parameters, this function
/// will fetch root folders from either Radarr (for movies) or Sonarr (for TV shows).
///
/// # Arguments
///
/// * `State(state)`: The application state containing clients for Radarr and Sonarr.
/// * `Query(params)`: The query parameters containing the media type.
///
/// # Returns
///
/// An HTTP response with the root folders in JSON format for the specified media type,
/// or a bad request response if the media type is invalid.
///
/// # Errors
///
/// This function will return a `BAD_REQUEST` status code if the media type is not "movie" or "tv_show".
pub async fn get_root_folder_handler(State(state): State<AppState>, Query(params): Query<MediaTypePayload>) -> impl IntoResponse {
    match params.media_type.as_str() {
        "movie" => (StatusCode::OK, Json(state.radarr_client.client.get_root_folders().await.unwrap())).into_response(),
        "tv_show" => (StatusCode::OK, Json(state.sonarr_client.client.get_root_folders().await.unwrap())).into_response(),
        _ => (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid media type" }))).into_response(),
    }
}
