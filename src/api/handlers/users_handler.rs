use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
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
    let stored_user = user_service::add_user(State(db), user)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok((StatusCode::CREATED, Json(stored_user)))
}

async fn get_all_users(
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    let stored_user_vec = user_service::get_all_users(State(db))
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok((StatusCode::OK, Json(stored_user_vec)))
}

// Get user
async fn get_user(
    State(db): State<DatabaseConnection>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!("Get user by id - {}", key);

    let stored_user = user_service::get_user_by_key(State(db), key)
        .await
        .map_err(|e| {
            warn!(" Error - {}", e.to_string());
            if e.to_string().contains("RecordNotFound") {
                AppError::ResourceNotFound
            } else {
                AppError::InternalServerError
            }
        })?;

    Ok((StatusCode::OK, Json(stored_user)))
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
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let response = user_service::delete_user_by_key(State(db), key)
        .await
        .map_err(|_| AppError::InternalServerError)?;
    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap()
        .into_response())
}

// Router function for hello handler
pub fn routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", post(create_user).get(get_all_users))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}
