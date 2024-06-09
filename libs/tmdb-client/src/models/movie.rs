use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub overview: String,
    pub homepage: String,
    pub popularity: f32,
    pub status: String,
    pub release_date: String,
    pub runtime: Option<u32>,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub vote_average: f32,
    pub vote_count: u32,
}
