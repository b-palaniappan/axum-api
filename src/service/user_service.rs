use std::error::Error;

use axum::extract::State;
use chrono::Utc;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, TransactionTrait};
use sea_orm::ActiveValue::Set;
use serde_json::json;
use tracing::info;

use crate::api::model::users::{CreateUser, StoredUser};
use crate::db::entity::address;
use crate::db::entity::users::ActiveModel;
use crate::service::location_service::get_geo_location;

pub async fn add_user(State(db): State<DatabaseConnection>, create_user: CreateUser) -> Result<StoredUser, Box<dyn Error>> {
    let now = Utc::now().naive_utc();
    let user = ActiveModel {
        key: Set(nanoid!().into_bytes()),
        first_name: Set(Some(create_user.first_name)),
        last_name: Set(create_user.last_name),
        email: Set(create_user.email),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    // Call location service to get get location info.
    // let location_position = get_geo_location(&create_user.address_line_one).await;
    //
    // let geocode = if let Ok(position) = location_position {
    //     info!("Location Position - {:?}", &position);
    //     json!({
    //         "lat": &position.lat,
    //         "lng": &position.lng,
    //     })
    // } else {
    //     warn!("Error getting position.");
    //     json!({
    //         "lat": 0,
    //         "lng": 0,
    //     })
    // };

    // This works.. but getting error for above code.
    let location_position = get_geo_location(&create_user.address_line_one).await?;
    let geocode = json!({
        "lat": &location_position.lat,
        "lng": &location_position.lng,
    });


    let transaction_resp = db.transaction::<_, (ActiveModel, address::ActiveModel), DbErr>(|txn| {
        Box::pin(async move {
            let user_resp = user.save(txn).await?;
            info!("Response in service -> {:?}", &user_resp);

            let user_id = &user_resp.id;

            let address = address::ActiveModel {
                key: Set(nanoid!().into_bytes()),
                user_id: Set(*user_id.as_ref()),
                line_one: Set(create_user.address_line_one),
                line_two: Set(create_user.address_line_two),
                city: Set(create_user.city),
                state: Set(create_user.state),
                country: Set(create_user.country),
                geocode: Set(geocode),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };

            let address_resp = address.save(txn).await?;
            Ok((user_resp, address_resp))
        })
    }).await?;

    let (saved_user, saved_address) = transaction_resp;

    Ok(StoredUser {
        id: saved_user.id.to_owned().unwrap() as i64,
        key: String::from_utf8_lossy(&saved_user.key.to_owned().unwrap()).to_string(),
        first_name: saved_user.first_name.to_owned().unwrap().unwrap_or_else(|| String::from("")),
        last_name: saved_user.last_name.to_owned().unwrap(),
        email: saved_user.email.to_owned().unwrap(),
        address_line_one: saved_address.line_one.to_owned().unwrap(),
        address_line_tow: saved_address.line_two.to_owned().unwrap(),
        city: saved_address.city.to_owned().unwrap(),
        state: saved_address.state.to_owned().unwrap(),
        country: saved_address.country.to_owned().unwrap(),
    })
}
