use anyhow::{Ok, Result};
use reqwest::Method;
use serde_json::Value;

use crate::{CreateMovieBody, Movie, MovieRenames, RenameMovieFilesBody, ServarrClient};

#[derive(Debug, Clone)]
pub struct RadarrClient {
    pub client: ServarrClient,
}

impl RadarrClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self { client: ServarrClient::new(base_url, api_key) }
    }

    pub async fn create_movie(&self, tmdb_id: u32, title: &str, quality_profile_id: u32, root_folder: &str) -> Result<Movie> {
        let tag = self.client.upsert_tag("original").await?;

        let movie_payload = serde_json::to_string(&CreateMovieBody {
            title: title.into(),
            tmdb_id,
            quality_profile_id,
            root_folder_path: root_folder.into(),
            monitored: true,
            minimum_availability: "announced".into(),
            tags: vec![tag.id],
        })
        .unwrap();

        let create_movie_url = self.client.build_url("movie");
        self.client.request::<Movie>(Method::POST, create_movie_url.as_str(), Some(movie_payload)).await
    }

    pub async fn scan_rename_movie(&self, movie_id: u32) -> Result<()> {
        let scan_command_id = self.scan_movie(movie_id).await?;
        self.client.await_command_completion(scan_command_id).await?;

        let movie_file_id = self.get_rename_file_ids(movie_id).await?;
        let rename_command_id = self.rename_files(movie_id, movie_file_id).await?;
        self.client.await_command_completion(rename_command_id).await?;

        Ok(())
    }

    async fn scan_movie(&self, movie_id: u32) -> Result<u32> {
        let url = self.client.build_url("command");
        let body = format!("{{ \"name\": \"RescanMovie\", \"movieId\": {} }}", movie_id);
        let res = self.client.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap() as u32)
    }

    async fn get_rename_file_ids(&self, movie_id: u32) -> Result<Vec<u32>> {
        let url = self.client.build_url_with_params("rename", &[("movieId", movie_id.to_string())])?;
        let res = self.client.request::<Vec<MovieRenames>>(Method::GET, url.as_str(), None).await?;
        Ok(res.iter().map(|x| x.movie_file_id).collect::<Vec<_>>())
    }

    async fn rename_files(&self, movie_id: u32, files: Vec<u32>) -> Result<u32> {
        let url = self.client.build_url("command");
        let body = serde_json::to_string(&RenameMovieFilesBody { name: "RenameFiles".into(), movie_id, files }).unwrap();
        let res = self.client.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap() as u32)
    }
}
