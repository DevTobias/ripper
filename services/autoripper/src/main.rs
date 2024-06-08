use axum::http::{header, HeaderValue, Method};
use axum::{routing::get, Router};
use serde::Deserialize;
use servarr_clients::{JellyfinClient, RadarrClient, SonarrClient};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{fmt::Debug, net::SocketAddr};
use tmdb_client::TmdbClient;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, Level};

mod handler;

#[derive(Debug, Clone, Deserialize)]
struct Config {
    makemkv_command: String,
    handbrake_command: String,
    output_dir: String,
    encoding_profiles_path: String,
    tmdb_key: String,
    radarr_endpoint: String,
    radarr_api_key: String,
    sonarr_endpoint: String,
    sonarr_api_key: String,
    jellyfin_endpoint: String,
    jellyfin_api_key: String,
    origin: String,
    remote_host: String,
    remote_user: String,
    remote_password: String,
}

#[derive(Debug, Clone)]
struct AppState {
    encoding_profiles_path: String,
    makemkv_command: String,
    handbrake_command: String,
    output_dir: String,
    tmdb_client: TmdbClient,
    makemkv_mutex: Arc<Mutex<()>>,
    radarr_client: RadarrClient,
    sonarr_client: SonarrClient,
    jellyfin_client: JellyfinClient,
    remote_host: String,
    remote_user: String,
    remote_password: String,
}

#[tokio::main]
async fn main() {
    let mut contents = String::new();
    File::open("config.json").unwrap().read_to_string(&mut contents).unwrap();
    let config: Config = serde_json::from_str(&contents).unwrap();

    let state = AppState {
        makemkv_command: config.makemkv_command,
        handbrake_command: config.handbrake_command,

        output_dir: config.output_dir,
        encoding_profiles_path: config.encoding_profiles_path,

        makemkv_mutex: Arc::new(Mutex::new(())),

        tmdb_client: TmdbClient::new(&config.tmdb_key),
        radarr_client: RadarrClient::new(&config.radarr_endpoint, &config.radarr_api_key),
        sonarr_client: SonarrClient::new(&config.sonarr_endpoint, &config.sonarr_api_key),
        jellyfin_client: JellyfinClient::new(&config.jellyfin_endpoint, &config.jellyfin_api_key),

        remote_host: config.remote_host,
        remote_user: config.remote_user,
        remote_password: config.remote_password,
    };

    tracing_subscriber::fmt().with_target(false).with_max_level(Level::DEBUG).compact().init();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::DEBUG))
        .on_request(DefaultOnRequest::new().level(Level::DEBUG));

    let cors = CorsLayer::new()
        .allow_origin(config.origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT]);

    let metadata_routes = Router::new()
        .route("/search/movie", get(handler::search_movie_handler))
        .route("/search/tv", get(handler::search_tv_show_handler))
        .route("/movie/:id", get(handler::get_movie_details_handler))
        .route("/tv/:id", get(handler::get_tv_show_details_handler));

    let handbrake_routes = Router::new().route("/encoding-presets", get(handler::get_encoding_profiles_handler));

    let makemkv_routes = Router::new()
        .route("/devices", get(handler::get_devices_handler))
        .route("/titles/movie", get(handler::get_movie_titles_handler))
        .route("/titles/tv", get(handler::get_tv_show_titles_handler))
        .route("/rip", get(handler::rip_websocket_handler));

    let media_routes = Router::new()
        .route("/quality-profiles", get(handler::get_quality_profile_handler))
        .route("/root-folders", get(handler::get_root_folder_handler));

    // state.radarr_client.create_movie(746036, "The Fall Guy", 4, "/data/media/movies").await.ok();
    // state.radarr_client.scan_rename_movie(113).await.ok();

    // state.jellyfin_client.library_scan().await.ok();

    // state
    //     .sonarr_client
    //     .create_tv_show(371572, "House of the Dragon", "standard", 4, "/data/media/tv")
    //     .await
    //     .ok();
    // Create Season 01 folder -> Upload [Bluray-1080p]_S01E02 Sample 1920x1080.mkv files
    // state.sonarr_client.scan_rename_tv_show(91).await.ok();

    let app = Router::new()
        .nest_service("/", ServeDir::new("./frontend/dist"))
        .nest("/api/tmdb", metadata_routes)
        .nest("/api/handbrake", handbrake_routes)
        .nest("/api/makemkv", makemkv_routes)
        .nest("/api/management", media_routes)
        .layer(cors)
        .layer(trace_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
