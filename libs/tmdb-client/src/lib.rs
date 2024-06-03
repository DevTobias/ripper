use anyhow::{Context, Result};
use futures::future::try_join_all;
use reqwest::{Client, Url};
use tracing::info;

pub use models::{Episode, Movie, TvSeason, TvSeries};
pub use models::{GenericSearchResponse, MovieSearchResult, TvSeriesSearchResult};

pub mod models;

#[derive(Debug, Clone)]
pub struct TmdbClient {
    client: Client,
    api_key: String,
}

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";

impl TmdbClient {
    /// Creates a new `TmdbClient` with the provided API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authenticating with the TMDB API.
    ///
    /// # Returns
    ///
    /// A new instance of `TmdbClient`.
    ///
    /// # Examples
    ///
    /// ```
    /// let tmdb_client = TmdbClient::new("your_api_key_here");
    /// ```
    pub fn new(api_key: &str) -> Self {
        Self { client: Client::new(), api_key: api_key.to_string() }
    }

    /// Sends a request to the TMDB API and deserializes the response into the specified type.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to send the request to.
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
    /// let response: Result<GenericSearchResponse<MovieSearchResult>, Error> = tmdb_client.tmdb_request("search/movie?query=Inception&include_adult=false&language=en").await;
    /// ```
    async fn tmdb_request<T>(&self, endpoint: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.client
            .get(&format!("{}/{}", TMDB_BASE_URL, endpoint))
            .bearer_auth(&self.api_key)
            .send()
            .await
            .context(format!("could not fetch {}", endpoint))?
            .error_for_status()?
            .json::<T>()
            .await
            .context("could not deserialize response")
    }
}

impl TmdbClient {
    /// Searches for movies on TMDB based on the given query and language.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string.
    /// * `lang` - The language code for the search results.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `GenericSearchResponse<MovieSearchResult>` if the request is successful,
    /// or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if URL construction fails, or if the request fails.
    pub async fn search_movies(&self, query: &str, lang: &str) -> Result<GenericSearchResponse<MovieSearchResult>> {
        let mut url = Url::parse(TMDB_BASE_URL)?.join("/search/movie").context("could not parse URL")?;
        url.query_pairs_mut()
            .append_pair("include_adult", "false")
            .append_pair("query", query)
            .append_pair("language", lang);

        info!("Searching movies with query: {}", query);

        self.tmdb_request::<GenericSearchResponse<MovieSearchResult>>(url.as_str()).await
    }

    /// Searches for TV series on TMDB based on the given query and language.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string.
    /// * `lang` - The language code for the search results.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `GenericSearchResponse<TvSeriesSearchResult>` if the request is successful,
    /// or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if URL construction fails, or if the request fails.
    pub async fn search_tv_series(&self, query: &str, lang: &str) -> Result<GenericSearchResponse<TvSeriesSearchResult>> {
        let mut url = Url::parse(TMDB_BASE_URL)?.join("/search/tv").context("could not parse URL")?;
        url.query_pairs_mut()
            .append_pair("include_adult", "false")
            .append_pair("query", query)
            .append_pair("language", lang);

        info!("Searching tv shows with query: {}", query);

        self.tmdb_request::<GenericSearchResponse<TvSeriesSearchResult>>(url.as_str()).await
    }

    /// Retrieves details of a specific movie from TMDB based on the given movie ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The TMDB ID of the movie.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Movie` struct if the request is successful, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if URL construction fails, or if the request fails.
    pub async fn get_movie(&self, id: u32) -> Result<Movie> {
        let url = Url::parse(TMDB_BASE_URL)?.join(&format!("/movie/{}", id)).context("could not parse URL")?;
        info!("Fetching movie with id: {}", id);
        self.tmdb_request::<Movie>(url.as_str()).await
    }

    /// Fetches details of a TV series, including season details, from TMDB.
    ///
    /// # Arguments
    ///
    /// * `id` - The TMDB ID of the TV series.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `TvSeries` struct if the request is successful, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an error if URL construction fails, the request fails, or if fetching season details fails.
    pub async fn get_tv_series(&self, id: u32) -> Result<TvSeries> {
        let url = Url::parse(TMDB_BASE_URL)?.join(&format!("/tv/{}", id)).context("could not parse URL")?;

        info!("Fetching tv series with id: {}", id);
        let mut response = self.tmdb_request::<TvSeries>(url.as_str()).await?;

        let season_futures = (1..=response.last_episode_to_air.season_number).map(|season| self.get_season_details(id, season));
        response.seasons = try_join_all(season_futures).await.context("could not fetch season details")?;

        Ok(response)
    }

    /// Fetches details of a specific season of a TV series from TMDB.
    ///
    /// # Arguments
    ///
    /// * `id` - The TMDB ID of the TV series.
    /// * `season` - The season number.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `TvSeason` struct if the request is successful, or an error if the request fails.
    pub async fn get_season_details(&self, id: u32, season: u16) -> Result<TvSeason> {
        let url = Url::parse(TMDB_BASE_URL)?
            .join(&format!("tv/{}/season/{}", id, season))
            .context("could not parse URL")?;

        info!("Fetching tv season details for id {} and season {}", id, season);

        self.tmdb_request::<TvSeason>(url.as_str()).await
    }
}
