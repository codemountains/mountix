use crate::model::information::JsonInformationResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn info() -> impl IntoResponse {
    let json: JsonInformationResponse = Default::default();
    (StatusCode::OK, Json(json))
}
