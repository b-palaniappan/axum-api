use axum::extract::State;
use sea_orm::DatabaseConnection;

use crate::api::model::users::{CreateUser, PatchUser, UpdateUser};

pub async fn create_user(State(db): State<DatabaseConnection>, create_user: &CreateUser) {
    todo!()
}

pub async fn update_user(State(db): State<DatabaseConnection>, id: i64, update_user: &UpdateUser) {
    todo!()
}

pub async fn delete_user(State(db): State<DatabaseConnection>, id: i64) {
    todo!()
}

pub async fn patch_user(State(db): State<DatabaseConnection>, id: i64, patch_user: &PatchUser) {
    todo!()
}
