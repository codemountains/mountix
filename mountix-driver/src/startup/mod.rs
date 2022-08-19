use crate::module::Modules;
use crate::routes::health::{hc, hc_mongodb};
use crate::routes::information::info;
use crate::routes::mountain::{find_mountains, get_mountain};
use axum::http::Method;
use axum::{routing::get, Extension, Router};
use dotenv::dotenv;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub async fn startup(modules: Arc<Modules>) {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::OPTIONS, Method::HEAD])
        .allow_origin(Any);

    let hc_router = Router::new()
        .route("/", get(hc))
        .route("/mongo", get(hc_mongodb));

    let mountain_router = Router::new()
        .route("/", get(find_mountains))
        .route("/:id", get(get_mountain));

    let info_router = Router::new().route("/", get(info));

    let app = Router::new()
        .nest("/api/v1/", info_router)
        .nest("/api/v1/hc", hc_router)
        .nest("/api/v1/mountains", mountain_router)
        .layer(cors)
        .layer(Extension(modules));

    let addr = SocketAddr::from(init_addr());
    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
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

    (ip_addr, port)
}
