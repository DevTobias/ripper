use makemkv_core::{detect_devices, filter_tv_series_candidates, read_properties, rip_titles};

#[tokio::main]
async fn main() {
    /*let tmdb_key: String = std::env::var("TMDB_KEY").unwrap();

    let devices = detect_devices("../_examples/makemkvcon_device").unwrap();
    let mut reader = DiscReader::new(
        "../_examples/makemkvcon_movie",
        &devices[0].path,
        Vec::from(["deu", "eng"]),
        tmdb_key.as_str(),
    );

    reader.read_properties().unwrap();
    reader.filter_movie_candidates(447365).await.unwrap();
    // reader.filter_tv_series_candidates(94997, 1, Vec::from([1, 2])).await.unwrap();

    let json_disc = serde_json::to_string_pretty(&reader.disc).unwrap();
    fs::write("parsed.json", json_disc).expect("written file");

    rip_disc(
        "/Applications/MakeMKV.app/Contents/MacOS/makemkvcon",
        &devices[0].path,
        Vec::from([0]),
        "./ripper_output",
        &|step, step_details, progress| println!("{}: {} - {:.2}%", step, step_details, progress * 100.0),
    )
    .unwrap();*/

    rip_tv_series().await;
}

async fn rip_tv_series() {
    let tmdb_key = std::env::var("TMDB_KEY").unwrap();
    let command = "/Applications/MakeMKV.app/Contents/MacOS/makemkvcon";

    let output_dir = "./ripper_output";
    let langs = Vec::from(["deu", "eng"]);
    let season = 1;
    let episodes = Vec::from([1, 2]);
    let tmdb_id = 94997;

    let devices = detect_devices(command).unwrap();
    let device = devices[0].path.clone();

    let mut disc = read_properties(&command, &device).unwrap();
    disc = filter_tv_series_candidates(disc, langs, season, episodes, tmdb_id, tmdb_key.as_str())
        .await
        .unwrap();

    let main_features = disc.titles.iter().map(|title| title.id).collect::<Vec<_>>();

    rip_titles(command, &device, main_features, output_dir, &|step_title, step_details, progress, step| {
        println!("{}: {} - {:.2}% - {}", step_title, step_details, progress * 100.0, step)
    })
    .unwrap();
}
