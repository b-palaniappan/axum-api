use axum::{Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use sqlx::MySqlPool;
use tracing::{error, info};
use validator::Validate;

use crate::api::model::api_error::{ApiErrorResponse};
use crate::api::model::users::{CreateUser, StoredUser};
use crate::service::event_service;

async fn create_user(State(pool): State<MySqlPool>, Json(user): Json<CreateUser>) -> impl IntoResponse {
  info!("Create a new User");
  event_service::app_event(State(pool)).await;
  match user.validate() {
    Ok(_) => {
      Ok(Json(StoredUser {
        id: "".to_string(),
        first_name: "".to_string(),
        last_name: "".to_string(),
        email: "".to_string(),
        address_line_one: "".to_string(),
        address_line_tow: None,
        city: "".to_string(),
        state: "".to_string(),
        country: "".to_string(),
      }))
    },
    Err(err) => {
      error!("Error - {}", err);
      Err(Json(ApiErrorResponse {
        status: 0,
        time: "".to_string(),
        message: "".to_string(),
        debug_message: None,
        sub_errors: vec![],
      }))
    },
  }
}

// Router function for hello handler
pub fn routes() -> Router<MySqlPool> {
  Router::new()
    .route("/", post(create_user))
}
