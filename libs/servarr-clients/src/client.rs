use anyhow::{Context, Result};
use reqwest::{Client, Method, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::borrow::Borrow;
use tracing::info;

use crate::{QualityProfile, Rootfolder, Tag};

#[derive(Debug, Clone)]
pub struct ServarrClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ServarrClient {
    /// Creates a new `ServarrClient` with the provided API key and base path.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authenticating with the Servarr API.
    /// * `base_url` - The base URL for the Servarr API.
    ///
    /// # Returns
    ///
    /// A new instance of `ServarrClient`.
    ///
    /// # Examples
    ///
    /// ```
    /// let Servarr_client = ServarrClient::new("your_api_key_here", "http://localhost:7878");
    /// ```
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self { client: Client::new(), api_key: api_key.to_string(), base_url: base_url.to_string() }
    }

    /// Sends a request to the Servarr API and deserializes the response into the specified type.
    ///
    /// # Arguments
    ///
    /// * `url` - The API endpoint to send the request to.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response of type `T` if the request is successful,
    /// or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, the response status is not successful, or
    /// deserialization of the response fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let response: Result<GenericSearchResponse<MovieSearchResult>, Error> = Servarr_client.request("/movie/lookup/tmdb?tmdbId=566525").await;
    /// ```
    pub async fn request<T: DeserializeOwned>(&self, method: Method, url: &str, body: Option<String>) -> Result<T> {
        let mut builder = self.client.request(method, url);

        if let Some(body_content) = body {
            builder = builder.body(body_content).header("Content-Type", "application/json");
        }

        info!("Sending request to {}", url);

        let response = builder
            .header("X-Api-Key", &self.api_key)
            .send()
            .await
            .context(format!("could not fetch {}", url))?;

        let status = response.status();
        let text = response.text().await.context("could not read response body")?;

        if !status.is_success() {
            println!("Error response body: {}", text);
            return Err(anyhow::anyhow!("request to {} failed with status {}", url, status));
        }

        let result: Result<T, _> = serde_json::from_str(&text).context("could not deserialize response");

        match result {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("Error deserializing response body: {}", text);
                Err(e)
            }
        }
    }

    /// Constructs a URL by appending a given path to the base URL.
    ///
    /// This function takes a path as an input and appends it to the `base_url` of the object,
    /// returning the resulting full URL as a string.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be appended to the base URL.
    ///
    /// # Returns
    ///
    /// * A `String` containing the full URL.
    ///
    /// # Example
    ///
    /// ```
    /// let client = MyClient { base_url: "https://api.example.com".to_string() };
    /// let url = client.build_url("endpoint");
    /// assert_eq!(url, "https://api.example.com/endpoint");
    /// ```
    ///
    pub fn build_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path)
    }

    /// Constructs a URL with query parameters by appending a given path to the base URL.
    ///
    /// This function takes a path and an iterator of key-value pairs, appending the path to the `base_url`
    /// and adding the key-value pairs as query parameters to the resulting URL. It returns the constructed URL or an error
    /// if the URL cannot be parsed.
    ///
    /// # Arguments
    ///
    /// * `input` - The path to be appended to the base URL.
    /// * `iter` - An iterator of key-value pairs to be added as query parameters.
    ///
    /// # Type Parameters
    ///
    /// * `I` - An iterator that can be converted into an iterator of items that can be borrowed as key-value pairs.
    /// * `K` - A type that can be referenced as a string slice, representing the keys in the query parameters.
    /// * `V` - A type that can be referenced as a string slice, representing the values in the query parameters.
    ///
    /// # Returns
    ///
    /// * `Result<Url, anyhow::Error>` - The constructed URL or an error if the URL could not be parsed.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// let client = MyClient { base_url: "https://api.example.com".to_string() };
    /// let mut params = HashMap::new();
    /// params.insert("key1", "value1");
    /// params.insert("key2", "value2");
    ///
    /// let url = client.build_url_with_params("endpoint", &params).unwrap();
    /// assert_eq!(url.as_str(), "https://api.example.com/endpoint?key1=value1&key2=value2");
    /// ```
    ///
    pub fn build_url_with_params<I, K, V>(&self, input: &str, iter: I) -> Result<Url>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut url = Url::options().parse(&format!("{}/{}", self.base_url, input));

        if let Ok(ref mut url) = url {
            url.query_pairs_mut().extend_pairs(iter);
        }

        url.context("could not parse URL")
    }

    pub async fn get_quality_profiles(&self) -> Result<Vec<QualityProfile>> {
        let url = self.build_url("qualityprofile");
        self.request::<Vec<QualityProfile>>(Method::GET, url.as_str(), None).await
    }

    pub async fn get_root_folders(&self) -> Result<Vec<Rootfolder>> {
        let url = self.build_url("rootfolder");
        self.request::<Vec<Rootfolder>>(Method::GET, url.as_str(), None).await
    }

    pub async fn upsert_tag(&self, label: &str) -> Result<Tag> {
        let url = self.build_url("tag");
        self.request::<Tag>(Method::POST, url.as_str(), Some(format!("{{ \"label\": \"{}\" }}", label)))
            .await
    }

    pub async fn has_command_completed(&self, command_id: u32) -> Result<bool> {
        let url = self.build_url(&format!("command/{}", command_id));
        let res = self.request::<Value>(Method::GET, url.as_str(), None).await?;
        Ok(res["status"].as_str().unwrap() == "completed")
    }

    pub async fn await_command_completion(&self, command_id: u32) -> Result<()> {
        let mut completed_counter = 0;

        while !self.has_command_completed(command_id).await? {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            completed_counter += 1;
            if completed_counter > 10 {
                anyhow::bail!("Command did not complete in time")
            }
        }

        Ok(())
    }
}
