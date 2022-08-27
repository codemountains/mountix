use serde::Serialize;

pub mod information;
pub mod mountain;
pub mod surrounding_mountain;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonErrorResponse {
    messages: Vec<String>,
}

impl JsonErrorResponse {
    pub(crate) fn new(messages: Vec<String>) -> Self {
        Self { messages }
    }
}
