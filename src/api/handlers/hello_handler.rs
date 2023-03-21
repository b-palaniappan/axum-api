use axum::{Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use sqlx::MySqlPool;
use tracing::info;

// JSON handler
async fn handler_json(State(pool): State<MySqlPool>) -> impl IntoResponse {
    info!("From Json Handler inside handlers package");
    Json("Hello World in json")
}

// Router function for hello handler
pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/", get(handler_json))
}
