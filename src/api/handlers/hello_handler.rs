use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use tracing::info;

// JSON handler
async fn handler_json() -> impl IntoResponse {
    info!("From Json Handler inside handlers package");
    Json("Hello World in json")
}

// Router function for hello handler
pub fn routes() -> Router {
    Router::new()
        .route("/", get(handler_json))
}
