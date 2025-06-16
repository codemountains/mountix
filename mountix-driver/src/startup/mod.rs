use crate::module::Modules;
use crate::routes::health::{hc, hc_mongodb};
use crate::routes::information::info;
use crate::routes::mountain::{find_mountains, find_mountains_by_box, get_mountain};
use crate::routes::surrounding_mountain::find_surroundings;
use axum::http::Method;
use axum::{routing::get, Extension, Router};
use dotenvy::dotenv;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Level;

pub async fn startup(modules: Arc<Modules>) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::OPTIONS, Method::HEAD])
        .allow_origin(Any);

    let hc_router = Router::new()
        .route("/", get(hc))
        .route("/mongo", get(hc_mongodb));

    let mountain_router = Router::new()
        .route("/", get(find_mountains))
        .route("/{id}", get(get_mountain))
        .route("/{id}/surroundings", get(find_surroundings))
        .route("/geosearch", get(find_mountains_by_box));

    let info_router = Router::new().route("/", get(info));

    let app = Router::new()
        .nest("/api/v1/", info_router)
        .nest("/api/v1/hc", hc_router)
        .nest("/api/v1/mountains", mountain_router)
        .layer(cors)
        .layer(Extension(modules))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
                if request.uri().path().starts_with("/api/v1/hc") {
                    tracing::debug!(
                        headers = ?request.headers(),
                        method = ?request.method(),
                        uri = ?request.uri(),
                        "Received Health Check request."
                    );
                    tracing::span!(Level::DEBUG, "http-request")
                } else {
                    tracing::info!(
                        headers = ?request.headers(),
                        method = ?request.method(),
                        uri = ?request.uri(),
                        "Received HTTP request."
                    );
                    tracing::span!(Level::INFO, "http-request")
                }
            }),
        );

    let addr = SocketAddr::from(init_addr());
    tracing::info!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to address {}: {:?}", addr, e));

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();
}

fn init_addr() -> (IpAddr, u16) {
    let env_host = env::var_os("HOST").expect("HOST is undefined.");
    let ip_addr = env_host
        .into_string()
        .expect("HOST is invalid.")
        .parse::<IpAddr>()
        .expect("HOST is invalid.");

    let env_port = env::var_os("PORT").expect("PORT is undefined.");
    let port = env_port
        .into_string()
        .expect("PORT is invalid.")
        .parse::<u16>()
        .expect("PORT is invalid.");

    tracing::debug!("Init ip address.");
    (ip_addr, port)
}
