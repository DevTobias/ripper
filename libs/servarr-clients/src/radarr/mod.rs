mod client;
pub use client::RadarrClient;

mod models;
pub use models::{CreateMovieBody, Movie, MovieRenames, RenameMovieFilesBody};
