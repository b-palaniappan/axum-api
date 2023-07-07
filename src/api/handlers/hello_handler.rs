use axum::{Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::service::event_service;

// JSON handler
async fn handler_json(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    info!("From Json Handler inside handlers package");
    event_service::add_event(State(db)).await;
    Json("Hello World in json")
}

// Router function for hello handler
pub fn routes() -> Router<DatabaseConnection> {
    Router::new().route("/", get(handler_json))
}
