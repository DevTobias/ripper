use std::net::SocketAddr;

use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use tmdb_client::TmdbClient;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handler;

#[derive(Debug, Clone)]
struct AppState {
    command: String,
    origin: String,
    tmdb_client: TmdbClient,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

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
        .route("/api/devices", get(handler::get_devices_handler))
        .route("/api/encoding-presets", get(handler::get_encoding_profiles_handler))
        .route("/api/tmdb/search/movie", post(handler::search_movie_handler))
        .route("/api/tmdb/search/tv", post(handler::search_tv_series_handler))
        .route("/api/tmdb/movie", post(handler::get_movie_details_handler))
        .route("/api/tmdb/tv", post(handler::get_tv_details_handler))
        .route("/api/rip/ws", get(handler::rip_websocket_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

/*
use makemkv_core::{detect_devices, filter_tv_series_candidates, read_properties, rip_titles};

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
*/
