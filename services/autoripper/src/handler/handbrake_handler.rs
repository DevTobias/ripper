use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use handbrake_core::get_encoding_profiles;

use crate::AppState;

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
