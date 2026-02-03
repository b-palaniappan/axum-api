use axum::extract::State;
use sqlx::{Error, PgPool};

use crate::api::model::users::{CreateUser, StoredUser, UpdateUser};
use crate::db::entity::user_entity::User;
use crate::db::repository::user_repository;

pub async fn create_user(
    State(pool): State<PgPool>,
    create_user: &CreateUser,
) -> Result<Option<User>, Error> {
    let response = user_repository::create_user(State(pool), create_user).await;
    return match response {
        Ok(Some(u)) => Ok(Some(u)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn update_user(State(pool): State<PgPool>, id: String, update_user: &UpdateUser) -> bool {
    user_repository::update_user(State(pool), id, update_user).await
}

pub async fn delete_user(State(pool): State<PgPool>, id: String) -> bool {
    user_repository::delete_user(State(pool), id).await
}

pub async fn get_user_by_id(State(pool): State<PgPool>, id: String) -> Result<Option<User>, Error> {
    let response = user_repository::get_user_by_id(State(pool), id).await;
    return match response {
        Ok(Some(u)) => Ok(Some(u)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    };
}

pub async fn get_all_user(State(pool): State<PgPool>) -> Result<Vec<StoredUser>, Error> {
    let user_list = user_repository::get_all_user(State(pool)).await;
    match user_list {
        Ok(users) => Ok(users.into_iter().map(|u| StoredUser::from(u)).collect()),
        Err(e) => Err(e),
    }
}
