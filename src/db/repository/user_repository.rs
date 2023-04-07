use axum::extract::State;
use chrono::Utc;
use sqlx::{Error, MySqlPool, query, query_as, Row};
use tracing::{error, info};

use crate::api::model::users::{CreateUser, PatchUser, UpdateUser};
use crate::db::entity::user_entity::User;

pub async fn get_all_user(State(pool): State<MySqlPool>) -> Result<Vec<User>, Error> {
  let users = query_as::<_, User>("SELECT * FROM users WHERE deleted_at is null")
    .fetch_all(&pool)
    .await;
  return users;
}

pub async fn get_user_by_id(State(pool): State<MySqlPool>, id: i64) -> Result<Option<User>, Error> {
  let user = query_as::<_, User>("SELECT * FROM users WHERE id = ? AND deleted_at is null")
    .bind(id)
    .fetch_optional(&pool)
    .await;
  return user;
}

pub async fn create_user(State(pool): State<MySqlPool>, create_user: &CreateUser) -> Result<Option<User>, Error> {
  let row =  query("INSERT INTO users (first_name, last_name, email, created_at, updated_at) values (?, ?, ?, ?, ?)")
    .bind(&create_user.first_name)
    .bind(&create_user.last_name)
    .bind(&create_user.email)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(&pool)
    .await;

  return match row {
    Ok(u) => {
      info!("Inserted Row successfully - {:?}", u);
      let added_user = get_user_by_id(State(pool), u.last_insert_id() as i64).await;
      match added_user {
        Ok(Some(u)) => return Ok(Some(u)),
        Ok(None) => return Ok(None),
        Err(e) => Err(e),
      }
    },
    Err(e) => {
      error!("Error {}", e);
      Err(e)
    },
  };
}

pub async fn update_user(State(pool): State<MySqlPool>, id: i64, update_user: &UpdateUser) -> Result<Option<User>, Error> {
  let row = query_as::<_, User>("UPDATE users SET first_name=?1, last_name=?2, email=?3, updated_at=?4 WHERE id=?4 and deleted_at is not null")
    .bind(&update_user.first_name)
    .bind(&update_user.last_name)
    .bind(&update_user.email)
    .bind(Utc::now())
    .bind(id)
    .fetch_optional(&pool)
    .await;

  return row;
}

pub async fn delete_user(State(pool): State<MySqlPool>, id: i64) -> bool {
  let row = query("UPDATE users SET deleted_at=?1 WHERE id=?2")
    .bind(Utc::now())
    .bind(id)
    .fetch_optional(&pool)
    .await;

  return match row {
    Ok(_) => true,
    Err(_) => false,
  };
}

pub async fn patch_user(State(pool): State<MySqlPool>, id: i64, patch_user: PatchUser) {
  // todo: work in progress!!!
}
