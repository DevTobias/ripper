mod client;
pub use client::SonarrClient;

mod models;
pub use models::{CreateTvShowBody, RenameTvShowFilesBody, TvShow, TvShowRenames};
