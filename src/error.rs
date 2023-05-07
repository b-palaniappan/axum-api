use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum AppError {
    InternalServerError,
}

pub struct Error {
    pub status: u16,
    pub time: String,
    pub message: String,
    pub debug_message: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occurred",
            ),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
