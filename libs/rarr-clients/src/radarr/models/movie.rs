use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMoviePayload {
    pub id: u64,
    pub path: String,
}
