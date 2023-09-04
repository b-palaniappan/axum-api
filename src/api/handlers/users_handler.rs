use axum::{debug_handler, Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use sea_orm::DatabaseConnection;
use tracing::{info, warn};
use validator::Validate;

use crate::api::model::users::{CreateUser, UpdateUser};
use crate::error::AppError;
use crate::service::user_service;

// Create user
#[debug_handler]
async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(user): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    info!("Create a new User");

    user.validate().map_err(|err| {
        warn!("Validation error: {}", err);
        AppError::ValidationError {
            validation_error: err,
            object: "User".to_string(),
        }
    })?;
    info!("User payload: {:?}", user);

    let stored_user = user_service::add_user(State(db), user).await.map_err(|_| AppError::InternalServerError)?;
    info!("Response in controller - {:?}", &stored_user);

    Ok((StatusCode::CREATED, Json(stored_user)))
}

// Get user
// TODO: Need to be implemented
async fn get_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    info!("Get user by id - {}", id);
    Ok((StatusCode::OK, Json("Get user by id")).into_response())
}

// Update user
async fn update_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
    Json(user): Json<UpdateUser>,
) -> Result<Response, AppError> {
    info!("Update existing User with id - {}", id);
    Ok((StatusCode::OK, Json("Update user")).into_response())
}

// Patch user
// TODO: Need to be implemented

// Delete user
async fn delete_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    info!("Delete user by id - {}", id);
    Ok((StatusCode::NO_CONTENT, Json("Delete user by id")).into_response())
}

// Router function for hello handler
pub fn routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}
