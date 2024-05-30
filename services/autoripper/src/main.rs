use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use handler::{get_devices_handler, search_movie_handler, search_tv_series_handler};
use makemkv_core::{detect_devices, filter_tv_series_candidates, read_properties, rip_titles};
use tmdb_client::TmdbClient;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod handler;

#[derive(Debug, Clone)]
struct AppState {
    command: String,
    origin: String,
    tmdb_client: TmdbClient,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        // command: "/Applications/MakeMKV.app/Contents/MacOS/makemkvcon".to_string(),
        command: "../_examples/makemkvcon_device".to_string(),
        origin: "http://localhost:5173".to_string(),
        tmdb_client: TmdbClient::new(std::env::var("TMDB_KEY").unwrap().as_str()),
    };

    let cors = CorsLayer::new()
        .allow_origin(state.origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT]);

    let app = Router::new()
        .nest_service("/", ServeDir::new("./frontend/dist"))
        .route("/api/devices", get(get_devices_handler))
        .route("/api/search/movie", post(search_movie_handler))
        .route("/api/search/tv", post(search_tv_series_handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn _rip_tv_series() {
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
