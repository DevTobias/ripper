use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TvSeries {
    pub id: u32,
    pub original_name: String,
    pub overview: String,
    pub homepage: String,
    pub popularity: f32,
    pub status: String,
    pub first_air_date: String,
    pub last_air_date: String,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub vote_average: f32,
    pub vote_count: u32,
    pub number_of_episodes: u32,
    pub number_of_seasons: u32,
    pub last_episode_to_air: Episode,
    #[serde(skip_deserializing)]
    pub seasons: Vec<TvSeason>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TvSeason {
    pub id: u32,
    pub season_number: u16,
    pub name: String,
    pub overview: String,
    pub air_date: String,
    pub poster_path: Option<String>,
    pub vote_average: f32,
    pub episodes: Vec<Episode>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Episode {
    pub id: u32,
    pub name: String,
    pub overview: String,
    pub air_date: Option<String>,
    pub episode_number: u16,
    pub episode_type: String,
    #[serde(default)]
    pub runtime: Option<u32>,
    pub season_number: u16,
    pub vote_average: f32,
    pub vote_count: Option<u32>,
    pub still_path: Option<String>,
}
