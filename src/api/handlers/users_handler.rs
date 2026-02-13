use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{error, info};
use validator::Validate;

use crate::api::model::api_error::ApiErrorResponse;
use crate::api::model::users::{CreateUser, StoredUser, UpdateUser};
use crate::service::user_service;

// Create user
async fn create_user(
    State(pool): State<PgPool>,
    Json(user): Json<CreateUser>,
) -> impl IntoResponse {
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
                    status: 404,
                    time: Utc::now().to_string(),
                    message: "User not found".to_string(),
                    debug_message: None,
                    sub_errors: vec![],
                })),
                Err(err) => {
                    error!("Error - {}", err);
                    Err(Json(ApiErrorResponse {
                        status: 500,
                        time: Utc::now().to_string(),
                        message: "Internal server error".to_string(),
                        debug_message: None,
                        sub_errors: vec![],
                    }))
                }
            };
        }
        Err(err) => {
            error!("Error - {}", err);
            Err(Json(ApiErrorResponse {
                status: 500,
                time: Utc::now().to_string(),
                message: "Internal server error".to_string(),
                debug_message: None,
                sub_errors: vec![],
            }))
        }
    }
}

// Get user
async fn get_user(State(pool): State<PgPool>, Path(id): Path<String>) -> Response {
    info!("Get user by id - {}", id);
    let response = user_service::get_user_by_id(State(pool), id).await;
    return match response {
        Ok(Some(u)) => (
            StatusCode::CREATED,
            [(header::LOCATION, format!("/users/{}", u.id))],
            Json(StoredUser {
                id: u.id,
                first_name: u.first_name,
                last_name: u.last_name,
                email: u.email,
                address_line_one: "".to_string(),
                address_line_tow: None,
                city: "".to_string(),
                state: "".to_string(),
                country: "".to_string(),
            }),
        )
            .into_response(),
        Err(err) => {
            error!("Error - {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse {
                    status: 500,
                    time: Utc::now().to_string(),
                    message: "Internal server error".to_string(),
                    debug_message: None,
                    sub_errors: vec![],
                }),
            )
                .into_response()
        }
        _ => {
            error!("Error - User not found");
            (
                StatusCode::NOT_FOUND,
                Json(ApiErrorResponse {
                    status: 404,
                    time: Utc::now().to_string(),
                    message: "User not found".to_string(),
                    debug_message: None,
                    sub_errors: vec![],
                }),
            )
                .into_response()
        }
    };
}

// Update user
async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(user): Json<UpdateUser>,
) -> impl IntoResponse {
    info!("Update existing User with id - {}", id);
    user_service::update_user(State(pool), id, &user).await;
}

// Patch user
// TODO: Need to be implemented

// Delete user
async fn delete_user(State(pool): State<PgPool>, Path(id): Path<String>) -> impl IntoResponse {
    info!("Delete user by id - {}", id);
    user_service::delete_user(State(pool), id).await;
    (StatusCode::NO_CONTENT).into_response()
}

async fn get_all_users(
    State(pool): State<PgPool>,
    Path((page, limit)): Path<(i64, i64)>,
) -> Response {
    let all_users = user_service::get_all_user(State(pool)).await;
    match all_users {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorResponse {
                status: 500,
                time: Utc::now().to_string(),
                message: "Internal server error".to_string(),
                debug_message: None,
                sub_errors: vec![],
            }),
        )
            .into_response(),
    }
}

// Router function for hello handler
pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}
