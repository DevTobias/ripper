use anyhow::{Context, Ok, Result};
use reqwest::Client;
use tracing::info;

#[derive(Debug, Clone)]
pub struct JellyfinClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl JellyfinClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self { client: Client::new(), api_key: api_key.to_string(), base_url: base_url.to_string() }
    }

    pub async fn library_scan(&self) -> Result<()> {
        let url = format!("{}/Library/Refresh", self.base_url);
        info!("Sending request to {}", &url);

        self.client
            .post(&url)
            .header("Authorization", format!("MediaBrowser Token=\"{}\"", &self.api_key))
            .send()
            .await
            .context(format!("could not fetch {}", url))?;

        Ok(())
    }
}
