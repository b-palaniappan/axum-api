use axum::extract::State;
use chrono::Utc;
use nid::alphabet::Base64UrlAlphabet;
use nid::Nanoid;
use sqlx::{Error, PgPool, query, query_as, Row};
use tracing::info;

use crate::api::model::users::{CreateUser, PatchUser, UpdateUser};
use crate::db::entity::user_entity::User;

pub async fn get_all_user(State(pool): State<PgPool>, page: i64, limit: i64) -> Result<Vec<User>, Error> {
    let users = query_as::<_, User>("SELECT * FROM \"user\" WHERE deleted_at is null ORDER BY created_at DESC LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind((page - 1) * limit)
        .fetch_all(&pool)
        .await;
    return users;
}

pub async fn get_user_by_id(State(pool): State<PgPool>, id: String) -> Result<Option<User>, Error> {
    let user = query_as::<_, User>("SELECT * FROM \"user\" WHERE id = $1 AND deleted_at is null")
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

    let id: Nanoid<24, Base64UrlAlphabet> = Nanoid::new();
    let user = query!(
        "INSERT INTO \"user\" (id, first_name, last_name, email, password_hash) values ($1, $2, $3, $4, $5) RETURNING id",
        id.to_string(),
        &create_user.first_name,
        &create_user.last_name,
        &create_user.email,
        "password_hash".to_string()
    )
    .fetch_one(&mut *txn)
    .await;

    let address_id: Nanoid<24, Base64UrlAlphabet> = Nanoid::new();
    let user_id = &user.unwrap().id;
    let address_line_two = create_user.address_line_two.as_deref().unwrap_or("");
    let row = query!("INSERT INTO address (id, line_one, line_two, city, state, zip, country, user_id) values ($1, $2, $3, $4, $5, $6, $7, $8)",
        address_id.to_string(), &create_user.address_line_one, address_line_two, &create_user.city, &create_user.state, &create_user.zipcode, &create_user.country, user_id)
        .execute(&mut *txn)
        .await;

    txn.commit().await?;
    info!("Inserted Row successfully. User id {:?}", user_id);
    let added_user = get_user_by_id(State(pool), user_id.to_string()).await;
    match added_user {
        Ok(Some(u)) => Ok(Some(u)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn update_user(State(pool): State<PgPool>, id: String, update_user: &UpdateUser) -> bool {
    let update_user = query!("UPDATE \"user\" SET first_name=$1, last_name=$2, email=$3, updated_at=$4 WHERE id=$5 and deleted_at is null", 
        &update_user.first_name, &update_user.last_name, &update_user.email, Utc::now(), id)
        .execute(&pool)
        .await;
    match update_user {
        Ok(_) => true,
        Err(e) => false,
    }
}

pub async fn delete_user(State(pool): State<PgPool>, id: String) -> bool {
    let row = query("UPDATE \"user\" SET deleted_at=$1 WHERE id=$2")
        .bind(Utc::now())
        .bind(id)
        .fetch_optional(&pool)
        .await;

    return match row {
        Ok(_) => true,
        Err(_) => false,
    };
}

pub async fn patch_user(State(pool): State<PgPool>, id: String, patch_user: &PatchUser) {
    // todo: work in progress!!!
    todo!()
}
