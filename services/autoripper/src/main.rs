use axum::http::{header, HeaderValue, Method};
use axum::{routing::get, Router};
use std::sync::{Arc, Mutex};
use std::{fmt::Debug, net::SocketAddr};
use tmdb_client::TmdbClient;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, Level};

mod handler;

#[derive(Debug, Clone)]
struct AppState {
    encoding_profiles_path: String,
    makemkv_command: String,
    handbrake_command: String,
    output_dir: String,
    origin: String,
    tmdb_client: TmdbClient,
    makemkv_mutex: Arc<Mutex<()>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        makemkv_command: "/Applications/MakeMKV.app/Contents/MacOS/makemkvcon".to_string(),
        handbrake_command: "/Applications/HandBrakeCLI".to_string(),
        output_dir: "/Users/tobias.kaerst/Documents/projects/ripper/.output".to_string(),
        encoding_profiles_path: "/Users/tobias.kaerst/Documents/projects/ripper/.profiles".to_string(),
        origin: "http://192.168.178.47:5173".to_string(),
        tmdb_client: TmdbClient::new(std::env::var("TMDB_KEY").unwrap().as_str()),
        makemkv_mutex: Arc::new(Mutex::new(())),
    };

    tracing_subscriber::fmt().with_target(false).with_max_level(Level::DEBUG).compact().init();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::DEBUG))
        .on_request(DefaultOnRequest::new().level(Level::DEBUG));

    let cors = CorsLayer::new()
        .allow_origin(state.origin.parse::<HeaderValue>().unwrap())
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
        .route("/rip/movie/ws", get(handler::rip_movie_websocket_handler));

    let app = Router::new()
        .nest_service("/", ServeDir::new("./frontend/dist"))
        .nest("/api/tmdb", metadata_routes)
        .nest("/api/handbrake", handbrake_routes)
        .nest("/api/makemkv", makemkv_routes)
        .layer(cors)
        .layer(trace_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
