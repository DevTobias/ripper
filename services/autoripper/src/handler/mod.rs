pub mod metadata_handler;
pub use metadata_handler::{get_movie_details_handler, get_tv_show_details_handler, search_movie_handler, search_tv_show_handler};

pub mod ripping_handler;
pub use ripping_handler::rip_websocket_handler;

pub mod media_handler;
pub use media_handler::{get_encoding_profiles_handler, get_quality_profile_handler, get_root_folder_handler};

pub mod disc_handler;
pub use disc_handler::{get_devices_handler, get_movie_titles_handler, get_tv_show_titles_handler};
