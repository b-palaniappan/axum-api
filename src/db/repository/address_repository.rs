use axum::extract::State;
use chrono::Utc;
use sqlx::{query, MySqlPool};

use crate::api::model::users::{CreateUser, PatchUser, UpdateUser};

pub async fn create_address(State(pool): State<MySqlPool>, create_user: &CreateUser, user_id: i64) {
}

pub async fn update_address(State(pool): State<MySqlPool>, id: i64, update_user: &UpdateUser) {}

pub async fn delete_address(State(pool): State<MySqlPool>, id: i64) {}

pub async fn patch_address(State(pool): State<MySqlPool>, id: i64, patch_user: &PatchUser) {}
