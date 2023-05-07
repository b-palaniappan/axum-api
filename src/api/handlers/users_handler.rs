use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use sea_orm::DatabaseConnection;
use tracing::{error, info};
use validator::Validate;

use crate::api::model::api_error::ApiErrorResponse;
use crate::api::model::users::{CreateUser, StoredUser, UpdateUser};
use crate::service::user_service;

// Create user
async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(user): Json<CreateUser>,
) -> impl IntoResponse {
    info!("Create a new User");
}

// Get user
// TODO: Need to be implemented
async fn get_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    info!("Get user by id - {}", id);
    Json("Get user by id")
}

// Update user
async fn update_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
    Json(user): Json<UpdateUser>,
) -> impl IntoResponse {
    info!("Update existing User with id - {}", id);
    Json("Update user")
}

// Patch user
// TODO: Need to be implemented

// Delete user
async fn delete_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    info!("Delete user by id - {}", id);
    Json("Delete user by id")
}

// Router function for hello handler
pub fn routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user).put(update_user))
}
