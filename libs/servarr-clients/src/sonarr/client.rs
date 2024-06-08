use anyhow::Result;
use reqwest::Method;
use serde_json::Value;

use crate::{sonarr::models::AddOptions, CreateTvShowBody, RenameTvShowFilesBody, ServarrClient, TvShow, TvShowRenames};

#[derive(Debug, Clone)]
pub struct SonarrClient {
    pub client: ServarrClient,
}

impl SonarrClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self { client: ServarrClient::new(base_url, api_key) }
    }

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
        self.client.request::<TvShow>(Method::POST, create_tv_show_url.as_str(), Some(series_payload)).await
    }

    pub async fn scan_rename_tv_show(&self, series_id: u32) -> Result<()> {
        let scan_command_id = self.scan_tv_show(series_id).await?;
        self.client.await_command_completion(scan_command_id).await?;

        let episode_file_ids = self.get_rename_file_ids(series_id).await?;
        let rename_command_id = self.rename_files(series_id, episode_file_ids).await?;
        self.client.await_command_completion(rename_command_id).await?;

        Ok(())
    }

    async fn scan_tv_show(&self, series_id: u32) -> Result<u32> {
        let url = self.client.build_url("command");
        let body = format!("{{ \"name\": \"RefreshSeries\", \"seriesId\": {} }}", series_id);
        let res = self.client.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap() as u32)
    }

    async fn get_rename_file_ids(&self, series_id: u32) -> Result<Vec<u32>> {
        let url = self.client.build_url_with_params("rename", &[("seriesId", series_id.to_string())])?;
        let res = self.client.request::<Vec<TvShowRenames>>(Method::GET, url.as_str(), None).await?;
        Ok(res.iter().map(|x| x.episode_file_id).collect::<Vec<_>>())
    }

    async fn rename_files(&self, series_id: u32, files: Vec<u32>) -> Result<u32> {
        let url = self.client.build_url("command");
        let body = serde_json::to_string(&RenameTvShowFilesBody { name: "RenameFiles".into(), series_id, files }).unwrap();
        let res = self.client.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap() as u32)
    }
}
