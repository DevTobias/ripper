use makemkv_core::{detect_devices, Ripper};
use std::fs;

#[tokio::main]
async fn main() {
    let tmdb_key = std::env::var("TMDB_KEY").unwrap();

    let devices = detect_devices("../_examples/makemkvcon_device").unwrap();
    let mut ripper = Ripper::new("../_examples/makemkvcon_movie", &devices[0].path);

    ripper.read_properties().unwrap();
    ripper.filter_movie_candidates(Vec::from(["deu", "eng"]), 447365, tmdb_key.as_str()).await.unwrap();

    let json_disc = serde_json::to_string_pretty(&ripper.disc).unwrap();
    fs::write("parsed.json", json_disc).expect("written file");
}
