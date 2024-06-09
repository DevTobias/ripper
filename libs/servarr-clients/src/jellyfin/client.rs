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
    /// Creates a new instance of `JellyfinClient`.
    ///
    /// This function initializes the `JellyfinClient` with the provided base URL and API key.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Jellyfin server.
    /// * `api_key` - The API key for authenticating with the Jellyfin server.
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self { client: Client::new(), api_key: api_key.to_string(), base_url: base_url.to_string() }
    }

    /// Initiates a library scan on the Jellyfin server.
    ///
    /// This function sends a request to the Jellyfin server to refresh the library.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Returns an empty result on success, or an error if the request fails.
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
