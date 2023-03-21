use axum::{Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use sqlx::MySqlPool;
use tracing::info;

use crate::api::model::users;

async fn create_user(State(pool): State<MySqlPool>) -> impl IntoResponse {
  info!("Create a new user");
  Json(users::CreateUser {
    first_name: "John".to_owned(),
    last_name: "Doe".to_owned(),
    email: "john@c12.io".to_string(),
    address_line_one: "1100 Locust St".to_string(),
    address_line_tow: None,
    city: "Des Moines".to_string(),
    state: "IA".to_string(),
    country: "US".to_string(),
  })
}

// Router function for hello handler
pub fn routes() -> Router<MySqlPool> {
  Router::new()
    .route("/", get(create_user))
}
