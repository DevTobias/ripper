use axum::extract::{Path, State};
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::AppState;

#[derive(Deserialize)]
pub struct SearchPayload {
    query: String,
    lang: String,
}

#[derive(Deserialize)]
pub struct DetailsPayload {
    lang: String,
}

/// Handles search requests for movies based on the given query and language.
///
/// # Arguments
///
/// * `state` - The application state containing the TMDB client.
/// * `payload` - The search payload containing the query and language.
///
/// # Returns
///
/// A JSON response containing the search results or an error message.
pub async fn search_movie_handler(State(state): State<AppState>, Query(params): Query<SearchPayload>) -> impl IntoResponse {
    match state.tmdb_client.search_movies(&params.query, &params.lang).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => {
            error!("Failed to search movies: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Internal Server Error" }))).into_response()
        }
    }
}

/// Handles search requests for TV show based on the given query and language.
///
/// # Arguments
///
/// * `state` - The application state containing the TMDB client.
/// * `payload` - The search payload containing the query and language.
///
/// # Returns
///
/// A JSON response containing the search results or an error message.
pub async fn search_tv_show_handler(State(state): State<AppState>, Query(params): Query<SearchPayload>) -> impl IntoResponse {
    match state.tmdb_client.search_tv_series(&params.query, &params.lang).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => {
            error!("Failed to search TV series: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Internal Server Error" }))).into_response()
        }
    }
}

/// Retrieves details of a TV show based on the given ID.
///
/// # Arguments
///
/// * `state` - The application state containing the TMDB client.
/// * `payload` - The payload containing the TV series ID.
///
/// # Returns
///
/// A JSON response containing the TV series details or an error message.
pub async fn get_tv_show_details_handler(State(state): State<AppState>, Path(id): Path<u32>, Query(params): Query<DetailsPayload>) -> impl IntoResponse {
    match state.tmdb_client.get_tv_series(id, &params.lang).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => {
            error!("Failed to get TV series details: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Internal Server Error" }))).into_response()
        }
    }
}

/// Retrieves details of a movie based on the given ID.
///
/// # Arguments
///
/// * `state` - The application state containing the TMDB client.
/// * `payload` - The payload containing the movie ID.
///
/// # Returns
///
/// A JSON response containing the movie details or an error message.
pub async fn get_movie_details_handler(State(state): State<AppState>, Path(id): Path<u32>, Query(params): Query<DetailsPayload>) -> impl IntoResponse {
    match state.tmdb_client.get_movie(id, &params.lang).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(err) => {
            error!("Failed to get movie details: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Internal Server Error" }))).into_response()
        }
    }
}
