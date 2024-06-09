use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use makemkv_core::{detect_devices, filter_movie_main_features, filter_tv_series_main_features, read_disc_properties};

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
