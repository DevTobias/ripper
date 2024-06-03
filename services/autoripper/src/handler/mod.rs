pub mod metadata_handler;
pub use metadata_handler::get_movie_details_handler;
pub use metadata_handler::get_tv_show_details_handler;
pub use metadata_handler::search_movie_handler;
pub use metadata_handler::search_tv_show_handler;

pub mod handbrake_handler;
pub use handbrake_handler::get_encoding_profiles_handler;

pub mod makemkv_handler;
pub use makemkv_handler::get_devices_handler;
pub use makemkv_handler::get_movie_titles_handler;
pub use makemkv_handler::get_tv_show_titles_handler;
