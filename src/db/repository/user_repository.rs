use axum::extract::State;
use chrono::Utc;
use sqlx::{Error, PgPool, query, query_as, Row};
use tracing::{error, info};

use crate::api::model::users::{CreateUser, PatchUser, UpdateUser};
use crate::db::entity::user_entity::User;

pub async fn get_all_user(State(pool): State<PgPool>) -> Result<Vec<User>, Error> {
    let users = query_as::<_, User>("SELECT * FROM users WHERE deleted_at is null")
        .fetch_all(&pool)
        .await;
    return users;
}

pub async fn get_user_by_id(State(pool): State<PgPool>, id: i64) -> Result<Option<User>, Error> {
    let user = query_as::<_, User>("SELECT * FROM users WHERE id = ? AND deleted_at is null")
        .bind(id)
        .fetch_optional(&pool)
        .await;
    return user;
}

pub async fn create_user(
    State(pool): State<PgPool>,
    create_user: &CreateUser,
) -> Result<Option<User>, Error> {
    let mut txn = pool.begin().await?;

    let user = query!("INSERT INTO \"user\" (first_name, last_name, email) values ($1, $2, $3)",
        &create_user.first_name, &create_user.last_name, &create_user.email)
        .fetch_one(&mut *txn)
        .await;

    let user_id = &user.unwrap().get("id");
    let address_line_two = create_user.address_line_two.as_deref().unwrap_or("");
    let row = query!("INSERT INTO address (line_one, line_two, city, state, zip, country, user_id) values ($1, $2, $3, $4, $5, $6, $7)",
        &create_user.address_line_one, address_line_two, &create_user.city, &create_user.state, &create_user.zipcode, &create_user.country, user_id)
        .execute(&mut *txn)
        .await;

    txn.commit().await?;

    // return match &user {
    //     Ok(u) => {
    //         info!("Inserted Row successfully. User id {:?}", user_id);
    //         let added_user = get_user_by_id(State(pool), *user_id).await;
    //         match added_user {
    //             Ok(Some(u)) => return Ok(Some(u)),
    //             Ok(None) => return Ok(None),
    //             Err(e) => Err(e),
    //         }
    //     }
    //     Err(e) => {
    //         Err(e)
    //     }
    // };
    info!("Inserted Row successfully. User id {:?}", user_id);
    let added_user = get_user_by_id(State(pool), *user_id).await;
    match added_user {
        Ok(Some(u)) => Ok(Some(u)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn update_user(
    State(pool): State<PgPool>,
    id: i64,
    update_user: &UpdateUser,
) -> Result<Option<User>, Error> {
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

pub async fn delete_user(State(pool): State<PgPool>, id: i64) -> bool {
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

pub async fn patch_user(State(pool): State<PgPool>, id: i64, patch_user: &PatchUser) {
    // todo: work in progress!!!
    todo!()
}
