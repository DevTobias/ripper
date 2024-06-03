use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Movie {
    pub id: u32,
    pub original_title: String,
    pub overview: String,
    pub homepage: String,
    pub popularity: f32,
    pub status: String,
    pub release_date: String,
    pub runtime: u32,
    pub backdrop_path: String,
    pub poster_path: String,
    pub vote_average: f32,
    pub vote_count: u32,
}
