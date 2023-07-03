use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use sea_orm::DatabaseConnection;
use tracing::{info, warn};
use validator::{Validate, ValidationErrors};

use crate::api::model::users::{CreateUser, StoredUser, UpdateUser};
use crate::error::{ApiError, AppError};
use crate::service::user_service;

// Create user
async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(user): Json<CreateUser>,
) -> Result<Response, AppError> {
    info!("Create a new User");
    match user.validate() {
        Ok(_) => {
            info!("User payload: {:?}", user);
            let response = user_service::add_user(State(db), user).await;
            info!("Response in controller - {:?}", response);
            Ok((StatusCode::CREATED, Json(response)).into_response())
        }
        Err(err) => {
            warn!("Validation error: {}", err);
            Err(AppError::ValidationError {
                validation_error: err,
                object: "User".to_string(),
            })
        }
    }
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
