use crate::db::entity::event_entity::Event;
use axum::extract::State;
use chrono::Utc;
use sqlx::{query, MySqlPool};

// Create a new event.
pub async fn create_event(
    State(pool): State<MySqlPool>,
    event: &Event,
    info: String,
    user_id: i64,
) {
    let row = query("INSERT INTO users(type, name, info, time, user_id) values(?, ?, ?, ?, ?)")
        .bind(serde_json::to_string(&event.event_type).unwrap_or("".to_string()))
        .bind(serde_json::to_string(&event.event_name).unwrap_or("".to_string()))
        .bind(info)
        .bind(Utc::now())
        .bind(user_id)
        .execute(&pool)
        .await;
}
