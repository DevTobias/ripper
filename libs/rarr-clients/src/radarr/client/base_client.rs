use anyhow::{Context, Result};
use reqwest::{Client, Method, Url};
use std::borrow::Borrow;
use tracing::info;

#[derive(Debug, Clone)]
pub struct RadarrClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl RadarrClient {
    /// Creates a new `RadarrClient` with the provided API key and base path.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authenticating with the Radarr API.
    /// * `base_url` - The base URL for the Radarr API.
    ///
    /// # Returns
    ///
    /// A new instance of `RadarrClient`.
    ///
    /// # Examples
    ///
    /// ```
    /// let radarr_client = RadarrClient::new("your_api_key_here", "http://localhost:7878");
    /// ```
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self { client: Client::new(), api_key: api_key.to_string(), base_url: base_url.to_string() }
    }

    /// Sends a request to the Radarr API and deserializes the response into the specified type.
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
    /// let response: Result<GenericSearchResponse<MovieSearchResult>, Error> = radarr_client.request("/movie/lookup/tmdb?tmdbId=566525").await;
    /// ```
    pub async fn request<T>(&self, method: Method, url: &str, body: Option<String>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        info!("Fetching Radarr API {}", url);

        let mut builder = self.client.request(method, url);

        if body.is_some() {
            builder = builder.body(body.unwrap()).header("Content-Type", "application/json");
        }

        builder
            .header("X-Api-Key", &self.api_key)
            .send()
            .await
            .context(format!("could not fetch {}", url))?
            .error_for_status()?
            .json::<T>()
            .await
            .context("could not deserialize response")
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
}
