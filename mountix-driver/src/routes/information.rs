use crate::model::information::JsonInformationResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn info() -> impl IntoResponse {
    tracing::info!("Access information endpoint.");

    let json: JsonInformationResponse = Default::default();
    (StatusCode::OK, Json(json))
}
