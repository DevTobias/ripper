mod client;
mod jellyfin;
mod models;
mod radarr;
mod sonarr;

pub use client::ServarrClient;
pub use jellyfin::JellyfinClient;
pub use models::{QualityProfile, Rootfolder, Tag};
pub use radarr::{CreateMovieBody, Movie, MovieRenames, RadarrClient, RenameMovieFilesBody};
pub use sonarr::{CreateTvShowBody, RenameTvShowFilesBody, SonarrClient, TvShow, TvShowRenames};
