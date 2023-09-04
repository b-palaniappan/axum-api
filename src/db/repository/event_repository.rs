use axum::extract::State;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use tracing::info;

use crate::api::model::events::CreateEvent;
use crate::db::entity::events::{ActiveModel, Entity as Events};
use crate::db::entity::sea_orm_active_enums::Type;

pub async fn create_event(State(db): State<DatabaseConnection>, create_event: CreateEvent) {
    let event = ActiveModel {
        r#type: Set(Type::Auth),
        name: Set(create_event.name.to_owned()),
        ..Default::default()
    };
    let response = event.save(&db).await.unwrap();
    info!("Saved value -> {:?}", response);

    let event_found = Events::find_by_id(2 as u64).one(&db).await.unwrap();
    info!("Search value -> {:?}", event_found)
}
