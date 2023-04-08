use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use sqlx::MySqlPool;
use tracing::{error, info};
use validator::Validate;

use crate::api::model::api_error::ApiErrorResponse;
use crate::api::model::users::{CreateUser, StoredUser, UpdateUser};
use crate::service::user_service;

// Create user
async fn create_user(State(pool): State<MySqlPool>, Json(user): Json<CreateUser>) -> impl IntoResponse {
  info!("Create a new User");

  match user.validate() {
    Ok(_) => {
      let response = user_service::create_user(State(pool), &user).await;
      return match response {
        Ok(Some(u)) => Ok(Json(StoredUser {
          id: u.id,
          first_name: u.first_name,
          last_name: u.last_name,
          email: u.email,
          address_line_one: "".to_string(),
          address_line_tow: None,
          city: "".to_string(),
          state: "".to_string(),
          country: "".to_string(),
        })),
        Ok(None) => Err(Json(ApiErrorResponse {
          status: 0,
          time: "".to_string(),
          message: "".to_string(),
          debug_message: None,
          sub_errors: vec![],
        })),
        Err(err) => {
          error!("Error - {}", err);
          Err(Json(ApiErrorResponse {
            status: 0,
            time: "".to_string(),
            message: "".to_string(),
            debug_message: None,
            sub_errors: vec![],
          }))
        }
      };
    }
    Err(err) => {
      error!("Error - {}", err);
      Err(Json(ApiErrorResponse {
        status: 0,
        time: "".to_string(),
        message: "".to_string(),
        debug_message: None,
        sub_errors: vec![],
      }))
    }
  }
}

// Get user
// TODO: Need to be implemented
async fn get_user(State(pool): State<MySqlPool>, Path(id): Path<String>) -> impl IntoResponse {
  info!("Get user by id - {}", id);
  Json("Get user by id")
}

// Update user
async fn update_user(State(pool): State<MySqlPool>, Path(id): Path<String>, Json(user): Json<UpdateUser>) -> impl IntoResponse {
  info!("Update existing User with id - {}", id);
  Json("Update user")
}

// Patch user
// TODO: Need to be implemented

// Delete user
async fn delete_user(State(pool): State<MySqlPool>, Path(id): Path<String>) -> impl IntoResponse {
  info!("Delete user by id - {}", id);
  Json("Delete user by id")
}

// Router function for hello handler
pub fn routes() -> Router<MySqlPool> {
  Router::new()
    .route("/", post(create_user))
    .route("/:id", get(get_user).put(update_user))
}
