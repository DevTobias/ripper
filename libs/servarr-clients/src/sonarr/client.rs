use anyhow::Result;
use reqwest::Method;
use serde_json::Value;

use crate::{sonarr::models::AddOptions, CreateTvShowBody, RenameTvShowFilesBody, ServarrClient, TvShow, TvShowRenames};

#[derive(Debug, Clone)]
pub struct SonarrClient {
    pub client: ServarrClient,
}

impl SonarrClient {
    /// Creates a new instance of `SonarrClient`.
    ///
    /// This function initializes the `SonarrClient` with the provided base URL and API key.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Sonarr server.
    /// * `api_key` - The API key for authenticating with the Sonarr server.
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self { client: ServarrClient::new(base_url, api_key) }
    }

    /// Creates a new TV show entry in Sonarr.
    ///
    /// This function sends a request to create a new TV show in Sonarr using the provided TVDB ID,
    /// title, series type, quality profile ID, and root folder.
    ///
    /// # Arguments
    ///
    /// * `tvdb_id` - The TVDB ID of the TV show.
    /// * `title` - The title of the TV show.
    /// * `series_type` - The type of series (e.g., standard, daily).
    /// * `quality_profile_id` - The ID of the quality profile to use.
    /// * `root_folder` - The root folder path where the TV show files will be stored.
    ///
    /// # Returns
    ///
    /// * `Result<TvShow>` - The created TV show or an error if the request fails.
    pub async fn create_tv_show(&self, tvdb_id: u32, title: &str, series_type: &str, quality_profile_id: u32, root_folder: &str) -> Result<TvShow> {
        let tag = self.client.upsert_tag("original").await?;

        let series_payload = serde_json::to_string(&CreateTvShowBody {
            title: title.into(),
            root_folder_path: root_folder.into(),
            series_type: series_type.into(),
            tags: vec![tag.id],
            season_folder: true,
            monitored: true,
            quality_profile_id,
            tvdb_id,
            add_options: AddOptions { monitor: "all".into() },
        })
        .unwrap();

        let create_tv_show_url = self.client.build_url("series");
        let res = self.client.request::<TvShow>(Method::POST, create_tv_show_url.as_str(), Some(series_payload)).await;

        if let Ok(tv_show) = res {
            Ok(tv_show)
        } else {
            Ok(self.get_tv_show(tvdb_id).await?)
        }
    }

    /// Initiates a scan and renaming process for a TV show in Sonarr.
    ///
    /// This function sends requests to scan and rename a TV show based on its ID.
    ///
    /// # Arguments
    ///
    /// * `series_id` - The ID of the TV show to scan and rename.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An empty result on success, or an error if the request fails.
    pub async fn scan_rename_tv_show(&self, series_id: u32) -> Result<()> {
        let scan_command_id = self.scan_tv_show(series_id).await?;
        self.client.await_command_completion(scan_command_id).await?;

        let episode_file_ids = self.get_rename_file_ids(series_id).await?;
        let rename_command_id = self.rename_files(series_id, episode_file_ids).await?;
        self.client.await_command_completion(rename_command_id).await?;

        Ok(())
    }

    /// Retrieves TV show information from Sonarr based on TVDB ID.
    ///
    /// This function sends a request to fetch TV show details from Sonarr using the provided TVDB ID.
    ///
    /// # Arguments
    ///
    /// * `tvdb_id` - The TVDB ID of the TV show.
    ///
    /// # Returns
    ///
    /// * `Result<TvShow>` - The TV show details or an error if the request fails.
    async fn get_tv_show(&self, tvdb_id: u32) -> Result<TvShow> {
        let url = self.client.build_url_with_params("series", &[("tvdbId", tvdb_id.to_string())])?;
        let res = self.client.request::<Vec<TvShow>>(Method::GET, url.as_str(), None).await;
        Ok(res?.pop().unwrap())
    }

    /// Initiates a rescan for a specific TV show in Sonarr.
    ///
    /// This function sends a request to rescan a TV show in Sonarr based on its ID.
    ///
    /// # Arguments
    ///
    /// * `series_id` - The ID of the TV show to rescan.
    ///
    /// # Returns
    ///
    /// * `Result<u32>` - The command ID for the rescan operation or an error if the request fails.
    async fn scan_tv_show(&self, series_id: u32) -> Result<u32> {
        let url = self.client.build_url("command");
        let body = format!("{{ \"name\": \"RefreshSeries\", \"seriesId\": {} }}", series_id);
        let res = self.client.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap() as u32)
    }

    /// Retrieves the IDs of files to be renamed for a specific TV show in Sonarr.
    ///
    /// This function sends a request to get the file IDs that need to be renamed for the specified TV show.
    ///
    /// # Arguments
    ///
    /// * `series_id` - The ID of the TV show whose files need to be renamed.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u32>>` - A vector of file IDs to be renamed or an error if the request fails.
    async fn get_rename_file_ids(&self, series_id: u32) -> Result<Vec<u32>> {
        let url = self.client.build_url_with_params("rename", &[("seriesId", series_id.to_string())])?;
        let res = self.client.request::<Vec<TvShowRenames>>(Method::GET, url.as_str(), None).await?;
        Ok(res.iter().map(|x| x.episode_file_id).collect::<Vec<_>>())
    }

    /// Renames TV show files in Sonarr.
    ///
    /// This function sends a request to rename the specified files for a given TV show in Sonarr.
    ///
    /// # Arguments
    ///
    /// * `series_id` - The ID of the TV show whose files need to be renamed.
    /// * `files` - A vector of file IDs to be renamed.
    ///
    /// # Returns
    ///
    /// * `Result<u32>` - The command ID for the rename operation or an error if the request fails.
    async fn rename_files(&self, series_id: u32, files: Vec<u32>) -> Result<u32> {
        let url = self.client.build_url("command");
        let body = serde_json::to_string(&RenameTvShowFilesBody { name: "RenameFiles".into(), series_id, files }).unwrap();
        let res = self.client.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap() as u32)
    }
}
