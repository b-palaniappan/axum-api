use axum::extract::State;
use axum::Json;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, SecondsFormat, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, JsonValue};
use nanoid::nanoid;
use sea_orm::ActiveValue::Set;
use serde_json::json;
use tracing::info;
use crate::api::model::users::{CreateUser, StoredUser};
use crate::db::entity::{address, users};

pub async fn add_user(State(db): State<DatabaseConnection>, create_user: CreateUser) -> StoredUser {
    let user = users::ActiveModel {
        key: Set(nanoid!().into_bytes()),
        first_name: Set(Some(create_user.first_name)),
        last_name: Set(create_user.last_name),
        email: Set(create_user.email),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
        deleted_at: Set(None),
        ..Default::default()
    };

    let user_resp = user.save(&db).await.unwrap();
    info!("Response in service -> {:?}", user_resp);

    let user_id = user_resp.id.unwrap();

    let address = address::ActiveModel {
        key: Set(nanoid!().into_bytes()),
        user_id: Set(user_id),
        line_one: Set(create_user.address_line_one),
        line_two: Set(create_user.address_line_two),
        city: Set(create_user.city),
        state: Set(create_user.state),
        country: Set(create_user.country),
        geocode: Set(json!("")),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let address_resp = address.save(&db).await.unwrap();

    StoredUser {
        id: user_id as i64,
        key: String::from_utf8_lossy(&user_resp.key.unwrap()).to_string(),
        first_name: user_resp.first_name.unwrap().unwrap(),
        last_name: user_resp.last_name.unwrap(),
        email: user_resp.email.unwrap(),
        address_line_one: address_resp.line_one.unwrap(),
        address_line_tow: address_resp.line_two.unwrap(),
        city: address_resp.city.unwrap(),
        state: address_resp.state.unwrap(),
        country: address_resp.country.unwrap(),
    }

}
