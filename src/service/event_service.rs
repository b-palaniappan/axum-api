use axum::extract::State;
use sea_orm::DatabaseConnection;

pub async fn add_event(State(db): State<DatabaseConnection>) {}
