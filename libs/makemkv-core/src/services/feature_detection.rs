use anyhow::{Context, Result};
use tracing::debug;

use tmdb_client::TmdbClient;

use crate::Disc;

/// Filters the movie candidates on a disc based on audio language and runtime criteria.
///
/// This function fetches movie details from TMDB using the provided `tmdb_id`, then filters
/// the titles on the disc to include only those that:
/// - Have at least one audio stream.
/// - Have at least one audio stream with a language code present in the provided `langs`.
/// - Have a runtime within ±10% of the actual movie runtime from TMDB.
///
/// # Arguments
///
/// * `disc` - The `Disc` object containing a list of titles to be filtered.
/// * `langs` - A vector of language codes (`&str`) to filter the audio streams.
/// * `tmdb_id` - The TMDB ID of the movie to fetch details for.
/// * `client` - A reference to the `TmdbClient` used to fetch movie details.
///
/// # Returns
///
/// * `Result<Disc>` - A new `Disc` object with filtered titles based on the criteria, or an error if fetching movie details fails.
///
/// # Errors
///
/// Returns an error if fetching the movie details from TMDB fails.
///
/// # Examples
///
/// ```
/// let disc = ...; // Disc with titles
/// let langs = vec!["deu", "eng"];
/// let tmdb_id = 12345;
/// let client = TmdbClient::new();
///
/// let filtered_disc = filter_movie_candidates(disc, langs, tmdb_id, &client).await?;
/// ```
pub async fn filter_movie_main_features(disc: Disc, langs: &[&str], tmdb_id: u32, client: &TmdbClient) -> Result<Disc> {
    let movie = client.get_movie(tmdb_id, langs[0]).await.context("failed to fetch movie details")?;
    let actual_runtime = (movie.runtime * 60) as f32;

    let mut filtered_disc = disc.clone();
    filtered_disc.titles = disc
        .titles
        .iter()
        .filter(|title| {
            if title.audio_streams.is_empty() {
                debug!("skipping title {} because it has no audio streams", title.id);
                return false;
            }

            let satisfies_language = title.audio_streams.iter().any(|stream| langs.contains(&stream.lang_code.as_str()));

            if !satisfies_language {
                debug!("skipping title {} because it does not satisfy language requirements", title.id);
                return false;
            }

            (title.duration as f32 >= actual_runtime * 0.9) && (title.duration as f32 <= actual_runtime * 1.1)
        })
        .map(|title| title.to_owned())
        .collect();

    Ok(filtered_disc)
}

/// Filters the TV series candidates on a disc based on audio language and runtime criteria.
///
/// This function fetches TV series details from TMDB using the provided `tmdb_id`, then filters
/// the titles on the disc to include only those that:
/// - Have at least one audio stream.
/// - Have at least one audio stream with a language code present in the provided `langs`.
/// - Have a runtime within ±10% of the actual episode runtimes from TMDB for the specified season and episodes.
///
/// # Arguments
///
/// * `disc` - The `Disc` object containing a list of titles to be filtered.
/// * `langs` - A slice of language codes (`&[&str]`) to filter the audio streams.
/// * `season` - The season number of the TV series to fetch details for.
/// * `episodes` - A slice of episode numbers (`&[u16]`) to filter the runtimes.
/// * `tmdb_id` - The TMDB ID of the TV series to fetch details for.
/// * `client` - A reference to the `TmdbClient` used to fetch TV series details.
///
/// # Returns
///
/// * `Result<Disc>` - A new `Disc` object with filtered titles based on the criteria, or an error if fetching TV series details fails.
///
/// # Errors
///
/// Returns an error if fetching the TV series details from TMDB fails or if the specified season is not found.
///
/// # Examples
///
/// ```
/// let disc = ...; // Disc with titles
/// let langs = ["deu", "eng"];
/// let season = 1;
/// let episodes = [1, 2, 3];
/// let tmdb_id = 12345;
/// let client = TmdbClient::new();
///
/// let filtered_disc = filter_tv_series_candidates(disc, &langs, season, &episodes, tmdb_id, &client).await?;
/// ```
pub async fn filter_tv_series_main_features(disc: Disc, langs: &[&str], season: u16, episodes: &[u16], tmdb_id: u32, client: &TmdbClient) -> Result<Disc> {
    let tv_series = client.get_tv_series(tmdb_id, langs[0]).await.context("failed to fetch TV series details")?;

    let episode_runtimes: Vec<f32> = tv_series
        .seasons
        .get((season - 1) as usize)
        .context("season not found")?
        .episodes
        .iter()
        .filter_map(|episode| if episodes.contains(&episode.episode_number) { Some((episode.runtime.unwrap_or(0) * 60) as f32) } else { None })
        .collect();

    let mut filtered_disc = disc.clone();
    filtered_disc.titles = disc
        .titles
        .iter()
        .filter(|title| {
            if title.audio_streams.is_empty() {
                debug!("skipping title {} because it has no audio streams", title.id);
                return false;
            }

            let satisfies_language = title.audio_streams.iter().any(|stream| langs.contains(&stream.lang_code.as_str()));

            if !satisfies_language {
                debug!("skipping title {} because it does not satisfy language requirements", title.id);
                return false;
            }

            episode_runtimes
                .iter()
                .any(|&runtime| (title.duration as f32 >= runtime * 0.9) && (title.duration as f32 <= runtime * 1.1))
        })
        .map(|title| title.to_owned())
        .collect();

    Ok(filtered_disc)
}
