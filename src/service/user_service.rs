use axum::extract::State;
use sqlx::{Error, MySqlPool};

use crate::api::model::users::{CreateUser, UpdateUser};
use crate::db::entity::user_entity::User;
use crate::db::repository::user_repository;

pub async fn create_user(
    State(pool): State<MySqlPool>,
    create_user: &CreateUser,
) -> Result<Option<User>, Error> {
    let response = user_repository::create_user(State(pool), create_user).await;
    return match response {
        Ok(Some(u)) => Ok(Some(u)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn update_user(State(pool): State<MySqlPool>, id: i64, update_user: &UpdateUser) -> bool {
    let response = user_repository::update_user(State(pool), id, update_user).await;
    return match response {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(_) => false,
    };
}
