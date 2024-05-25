use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct TvSeries {
    pub id: u32,
    pub original_name: String,
    pub overview: String,
    pub homepage: String,
    pub popularity: f32,
    pub status: String,
    pub first_air_date: String,
    pub last_air_date: String,
    pub backdrop_path: String,
    pub poster_path: String,
    pub vote_average: f32,
    pub vote_count: u32,
    pub number_of_episodes: u32,
    pub number_of_seasons: u32,
    pub last_episode_to_air: Episode,
    #[serde(skip_deserializing)]
    pub seasons: Vec<TvSeason>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TvSeason {
    pub id: u32,
    pub season_number: u16,
    pub name: String,
    pub overview: String,
    pub air_date: String,
    pub poster_path: String,
    pub vote_average: f32,
    pub episodes: Vec<Episode>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Episode {
    pub id: u32,
    pub name: String,
    pub overview: String,
    pub air_date: String,
    pub episode_number: u16,
    pub episode_type: String,
    #[serde(default)]
    pub runtime: u32,
    pub season_number: u16,
    pub vote_average: f32,
    pub vote_count: u32,
    pub still_path: String,
}
