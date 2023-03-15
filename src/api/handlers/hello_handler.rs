use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;

// JSON handler
async fn handler_json() -> impl IntoResponse {
    Json("Hello World in json")
}

// Router function for hello handler
pub fn routes() -> Router {
    Router::new()
        .route("/", get(handler_json))
}
