use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use std::{
    fmt::Debug,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tmdb_client::TmdbClient;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod error;
mod handler;

#[derive(Debug, Clone)]
struct AppState {
    command: String,
    output_dir: String,
    origin: String,
    tmdb_client: TmdbClient,
    makemkv_mutex: Arc<Mutex<()>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).with_max_level(Level::DEBUG).compact().init();

    let state = AppState {
        command: "/Applications/MakeMKV.app/Contents/MacOS/makemkvcon".to_string(),
        // command: "../_examples/makemkvcon_device".to_string(),
        output_dir: "./ripper_output".to_string(),
        origin: "http://localhost:5173".to_string(),
        tmdb_client: TmdbClient::new(std::env::var("TMDB_KEY").unwrap().as_str()),
        makemkv_mutex: Arc::new(Mutex::new(())),
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
        .route("/api/rip/movie/ws", get(handler::rip_movie_websocket_handler))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG))
                .on_request(DefaultOnRequest::new().level(Level::DEBUG)),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
