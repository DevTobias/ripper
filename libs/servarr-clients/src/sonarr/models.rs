use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TvShow {
    pub id: u32,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTvShowBody {
    pub title: String,
    #[serde(rename = "tvdbId")]
    pub tvdb_id: u32,
    #[serde(rename = "qualityProfileId")]
    pub quality_profile_id: u32,
    #[serde(rename = "rootFolderPath")]
    pub root_folder_path: String,
    #[serde(rename = "seriesType")]
    pub series_type: String,
    #[serde(rename = "seasonFolder")]
    pub season_folder: bool,
    pub tags: Vec<u32>,
    pub monitored: bool,
    #[serde(rename = "addOptions")]
    pub add_options: AddOptions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddOptions {
    pub monitor: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TvShowRenames {
    #[serde(rename = "seriesId")]
    pub series_id: u32,
    #[serde(rename = "seasonNumber")]
    pub season_number: u32,
    #[serde(rename = "episodeNumbers")]
    pub episode_numbers: Vec<u32>,
    #[serde(rename = "episodeFileId")]
    pub episode_file_id: u32,
    #[serde(rename = "existingPath")]
    pub existing_path: String,
    #[serde(rename = "newPath")]
    pub new_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RenameTvShowFilesBody {
    pub name: String,
    #[serde(rename = "seriesId")]
    pub series_id: u32,
    pub files: Vec<u32>,
}
