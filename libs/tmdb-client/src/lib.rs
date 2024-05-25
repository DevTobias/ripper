use futures::future::join_all;
use reqwest::{Client, Error};

mod models;

pub struct TmdbClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl TmdbClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            base_url: "https://api.themoviedb.org/3".to_string(),
        }
    }

    async fn tmdb_request<T>(&self, url: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .json::<T>()
            .await
    }

    pub async fn get_movie(&self, id: u32) -> Result<models::Movie, Error> {
        let movie = self
            .tmdb_request::<models::Movie>(&format!("{}/movie/{}", self.base_url, id))
            .await?;

        Ok(movie)
    }

    pub async fn get_tv_series(&self, id: u32) -> Result<models::TvSeries, Error> {
        let mut response = self
            .tmdb_request::<models::TvSeries>(&format!("{}/tv/{}", self.base_url, id))
            .await?;

        let season_futures = (1..=response.last_episode_to_air.season_number)
            .map(|season| self.get_season_details(id, season));

        response.seasons = join_all(season_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(response)
    }

    async fn get_season_details(&self, id: u32, season: u16) -> Result<models::TvSeason, Error> {
        let response = self
            .tmdb_request::<models::TvSeason>(&format!(
                "{}/tv/{}/season/{}",
                self.base_url, id, season
            ))
            .await?;

        Ok(response)
    }
}
