use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub id: u32,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMovieBody {
    pub title: String,
    #[serde(rename = "tmdbId")]
    pub tmdb_id: u32,
    #[serde(rename = "rootFolderPath")]
    pub root_folder_path: String,
    pub monitored: bool,
    #[serde(rename = "qualityProfileId")]
    pub quality_profile_id: u32,
    #[serde(rename = "minimumAvailability")]
    pub minimum_availability: String,
    pub tags: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieRenames {
    #[serde(rename = "movieId")]
    pub movie_id: u32,
    #[serde(rename = "movieFileId")]
    pub movie_file_id: u32,
    #[serde(rename = "existingPath")]
    pub existing_path: String,
    #[serde(rename = "newPath")]
    pub new_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RenameMovieFilesBody {
    pub name: String,
    #[serde(rename = "movieId")]
    pub movie_id: u32,
    pub files: Vec<u32>,
}
