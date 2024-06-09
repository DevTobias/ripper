use anyhow::Result;
use reqwest::Method;
use serde_json::Value;

use crate::{CreateMovieBody, Movie, MovieRenames, RenameMovieFilesBody, ServarrClient};

#[derive(Debug, Clone)]
pub struct RadarrClient {
    pub client: ServarrClient,
}

impl RadarrClient {
    /// Creates a new instance of `RadarrClient`.
    ///
    /// This function initializes the `RadarrClient` with the provided base URL and API key.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Radarr server.
    /// * `api_key` - The API key for authenticating with the Radarr server.
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self { client: ServarrClient::new(base_url, api_key) }
    }

    /// Creates a new movie entry in Radarr.
    ///
    /// This function sends a request to create a new movie in Radarr using the provided TMDB ID,
    /// title, quality profile ID, and root folder.
    ///
    /// # Arguments
    ///
    /// * `tmdb_id` - The TMDB ID of the movie.
    /// * `title` - The title of the movie.
    /// * `quality_profile_id` - The ID of the quality profile to use.
    /// * `root_folder` - The root folder path where the movie files will be stored.
    ///
    /// # Returns
    ///
    /// * `Result<Movie>` - The created movie or an error if the request fails.
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
        let res = self.client.request::<Movie>(Method::POST, create_movie_url.as_str(), Some(movie_payload)).await;

        if let Ok(tv_show) = res {
            Ok(tv_show)
        } else {
            Ok(self.get_movie(tmdb_id).await?)
        }
    }

    /// Initiates a scan and renaming process for a movie in Radarr.
    ///
    /// This function sends requests to scan and rename a movie based on its ID.
    ///
    /// # Arguments
    ///
    /// * `movie_id` - The ID of the movie to scan and rename.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An empty result on success, or an error if the request fails.
    pub async fn scan_rename_movie(&self, movie_id: u32) -> Result<()> {
        let scan_command_id = self.scan_movie(movie_id).await?;
        self.client.await_command_completion(scan_command_id).await?;

        let movie_file_id = self.get_rename_file_ids(movie_id).await?;
        let rename_command_id = self.rename_files(movie_id, movie_file_id).await?;
        self.client.await_command_completion(rename_command_id).await?;

        Ok(())
    }

    /// Retrieves movie information from Radarr based on TMDB ID.
    ///
    /// This function sends a request to fetch movie details from Radarr using the provided TMDB ID.
    ///
    /// # Arguments
    ///
    /// * `tmdb_id` - The TMDB ID of the movie.
    ///
    /// # Returns
    ///
    /// * `Result<Movie>` - The movie details or an error if the request fails.
    async fn get_movie(&self, tmdb_id: u32) -> Result<Movie> {
        let url = self.client.build_url_with_params("movie", &[("tmdbId", tmdb_id.to_string())])?;
        let res = self.client.request::<Vec<Movie>>(Method::GET, url.as_str(), None).await;
        Ok(res?.pop().unwrap())
    }

    /// Initiates a rescan for a specific movie in Radarr.
    ///
    /// This function sends a request to rescan a movie in Radarr based on its ID.
    ///
    /// # Arguments
    ///
    /// * `movie_id` - The ID of the movie to rescan.
    ///
    /// # Returns
    ///
    /// * `Result<u32>` - The command ID for the rescan operation or an error if the request fails.
    async fn scan_movie(&self, movie_id: u32) -> Result<u32> {
        let url = self.client.build_url("command");
        let body = format!("{{ \"name\": \"RescanMovie\", \"movieId\": {} }}", movie_id);
        let res = self.client.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap() as u32)
    }

    /// Retrieves the IDs of files to be renamed for a specific movie in Radarr.
    ///
    /// This function sends a request to get the file IDs that need to be renamed for the specified movie.
    ///
    /// # Arguments
    ///
    /// * `movie_id` - The ID of the movie whose files need to be renamed.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u32>>` - A vector of file IDs to be renamed or an error if the request fails.
    async fn get_rename_file_ids(&self, movie_id: u32) -> Result<Vec<u32>> {
        let url = self.client.build_url_with_params("rename", &[("movieId", movie_id.to_string())])?;
        let res = self.client.request::<Vec<MovieRenames>>(Method::GET, url.as_str(), None).await?;
        Ok(res.iter().map(|x| x.movie_file_id).collect::<Vec<_>>())
    }

    /// Renames movie files in Radarr.
    ///
    /// This function sends a request to rename the specified files for a given movie in Radarr.
    ///
    /// # Arguments
    ///
    /// * `movie_id` - The ID of the movie whose files need to be renamed.
    /// * `files` - A vector of file IDs to be renamed.
    ///
    /// # Returns
    ///
    /// * `Result<u32>` - The command ID for the rename operation or an error if the request fails.
    async fn rename_files(&self, movie_id: u32, files: Vec<u32>) -> Result<u32> {
        let url = self.client.build_url("command");
        let body = serde_json::to_string(&RenameMovieFilesBody { name: "RenameFiles".into(), movie_id, files }).unwrap();
        let res = self.client.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap() as u32)
    }
}
