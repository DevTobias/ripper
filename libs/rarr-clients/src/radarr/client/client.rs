use anyhow::Result;
use reqwest::Method;
use serde_json::Value;

use crate::{CreateMoviePayload, QualityProfile, RadarrClient, Rootfolder, Tag};

impl RadarrClient {
    pub async fn get_quality_profiles(&self) -> Result<Vec<QualityProfile>> {
        let url = self.build_url("qualityprofile");
        self.request::<Vec<QualityProfile>>(Method::GET, url.as_str(), None).await
    }

    pub async fn get_root_folders(&self) -> Result<Vec<Rootfolder>> {
        let url = self.build_url("rootfolder");
        self.request::<Vec<Rootfolder>>(Method::GET, url.as_str(), None).await
    }

    pub async fn create_movie(&self, tmdb_id: u32, quality_profile_id: u32, root_folder: &str) -> Result<CreateMoviePayload> {
        let mut movie = self.search_movie(tmdb_id).await?;
        let tag = self.upsert_tag("original").await?;

        let create_movie_url = self.build_url("movie");

        movie["qualityProfileId"] = quality_profile_id.into();
        movie["rootFolderPath"] = root_folder.into();
        movie["monitored"] = true.into();
        movie["minimumAvailability"] = "announced".into();
        movie["tags"] = vec![tag.id].into();

        self.request::<CreateMoviePayload>(Method::POST, create_movie_url.as_str(), Some(serde_json::to_string(&movie).unwrap()))
            .await
    }

    pub async fn scan_rename_movie(&self, movie_id: u64) -> Result<()> {
        let scan_command_id = self.scan_movie(movie_id).await?;
        self.await_command_completion(scan_command_id).await?;

        let movie_file_id = self.get_movie_file_id(movie_id).await?;
        let rename_command_id = self.rename_file(movie_file_id, movie_id).await?;
        self.await_command_completion(rename_command_id).await?;

        Ok(())
    }

    async fn search_movie(&self, tmdb_id: u32) -> Result<Value> {
        let url = self.build_url_with_params("movie/lookup/tmdb", &[("tmdbid", tmdb_id.to_string())])?;
        self.request::<Value>(Method::GET, url.as_str(), None).await
    }

    async fn upsert_tag(&self, label: &str) -> Result<Tag> {
        let url = self.build_url("tag");
        self.request::<Tag>(Method::POST, url.as_str(), Some(format!("{{ \"label\": \"{}\" }}", label)))
            .await
    }

    async fn get_movie_file_id(&self, movie_id: u64) -> Result<u64> {
        let url = self.build_url_with_params("moviefile", &[("movieId", movie_id.to_string())])?;
        let res = self.request::<Value>(Method::GET, url.as_str(), None).await?;
        Ok(res[0]["id"].as_u64().unwrap())
    }

    async fn scan_movie(&self, movie_id: u64) -> Result<u64> {
        let url = self.build_url("command");
        let body = format!("{{ \"name\": \"RescanMovie\", \"movieId\": {} }}", movie_id);
        let res = self.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap())
    }

    async fn rename_file(&self, movie_file_id: u64, movie_id: u64) -> Result<u64> {
        let url = self.build_url("command");
        let body = format!("{{ \"name\": \"RenameFiles\", \"files\": [{}], \"movieId\": {} }}", movie_file_id, movie_id);

        let res = self.request::<Value>(Method::POST, url.as_str(), Some(body)).await?;
        Ok(res["id"].as_u64().unwrap())
    }

    async fn has_command_completed(&self, command_id: u64) -> Result<bool> {
        let url = self.build_url(&format!("command/{}", command_id));
        let res = self.request::<Value>(Method::GET, url.as_str(), None).await?;
        Ok(res["status"].as_str().unwrap() == "completed")
    }

    async fn await_command_completion(&self, command_id: u64) -> Result<()> {
        let mut completed_counter = 0;

        while !self.has_command_completed(command_id).await? {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            completed_counter += 1;
            if completed_counter > 10 {
                anyhow::bail!("Command did not complete in time")
            }
        }

        Ok(())
    }
}
