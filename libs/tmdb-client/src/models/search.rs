use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GenericSearchResponse<T> {
    pub page: u32,
    pub total_results: u32,
    pub total_pages: u32,
    pub results: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MovieSearchResult {
    pub id: u32,
    pub title: String,
    pub overview: String,
    pub original_language: String,
    pub popularity: f32,
    pub release_date: String,
    pub poster_path: Option<String>,
    pub vote_average: f32,
    pub vote_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TvSeriesSearchResult {
    pub id: u32,
    pub name: String,
    pub overview: String,
    pub original_language: String,
    pub popularity: f32,
    pub first_air_date: String,
    pub poster_path: Option<String>,
    pub vote_average: f32,
    pub vote_count: u32,
}
