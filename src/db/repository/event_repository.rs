use axum::extract::State;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::Set;
use tracing::info;

use crate::db::entity::{events, sea_orm_active_enums::Type};
use crate::db::entity::events::Entity as Events;

pub async fn create_event(State(db): State<DatabaseConnection>) {
    let event = events::ActiveModel {
        r#type: Set(Type::Auth),
        name: Set("Hello World".to_owned()),
        ..Default::default()
    };
    let response = event.save(&db).await.unwrap();
    info!("Saved value -> {:?}", response);

    let event_found = Events::find_by_id(2 as u64).one(&db).await.unwrap();
    info!("Search value -> {:?}", event_found)
}
