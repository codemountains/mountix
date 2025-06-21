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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Shared lock to ensure environment variable tests run sequentially
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_init_addr_with_valid_env_vars() {
        let _lock = ENV_LOCK.lock().unwrap();

        // Set test environment variables
        std::env::set_var("HOST", "127.0.0.1");
        std::env::set_var("PORT", "8080");

        let (ip_addr, port) = init_addr();

        assert_eq!(ip_addr.to_string(), "127.0.0.1");
        assert_eq!(port, 8080);

        // Clean up environment variables
        std::env::remove_var("HOST");
        std::env::remove_var("PORT");
    }

    #[test]
    fn test_init_addr_with_different_host() {
        let _lock = ENV_LOCK.lock().unwrap();

        std::env::set_var("HOST", "0.0.0.0");
        std::env::set_var("PORT", "3000");

        let (ip_addr, port) = init_addr();

        assert_eq!(ip_addr.to_string(), "0.0.0.0");
        assert_eq!(port, 3000);

        std::env::remove_var("HOST");
        std::env::remove_var("PORT");
    }

    #[test]
    fn test_socket_addr_creation() {
        // Test SocketAddr creation from IP and port using manual construction
        // This avoids relying on environment variables that might be in an inconsistent state
        let ip_addr: IpAddr = "192.168.1.1".parse().unwrap();
        let port: u16 = 9000;
        let socket_addr = SocketAddr::from((ip_addr, port));

        assert_eq!(socket_addr.ip().to_string(), "192.168.1.1");
        assert_eq!(socket_addr.port(), 9000);
    }

    #[test]
    fn test_env_var_parsing() {
        let _lock = ENV_LOCK.lock().unwrap();

        // Test environment variable parsing for different scenarios
        std::env::set_var("HOST", "::1"); // IPv6 localhost
        std::env::set_var("PORT", "8443");

        let (ip_addr, port) = init_addr();

        assert_eq!(ip_addr.to_string(), "::1");
        assert_eq!(port, 8443);

        std::env::remove_var("HOST");
        std::env::remove_var("PORT");
    }

    // Note: Full startup tests would require actual server initialization
    // These tests focus on individual components and configuration
}
