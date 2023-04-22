use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Event {
    pub id: i64,
    pub event_type: EventType,
    pub event_name: EventName,
    pub event_time: DateTime<Utc>,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    User,
    Auth,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventName {
    // user event
    CreateUser,
    RegisterUser,
    UpdateEmail,
    ConfirmEmailAddress,

    // update event
    ResetPassword,
    UpdatePassword,
    ForgotPassword,
    InvalidPassword,
    // other event
}

impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            EventType::User => "User".to_string(),
            EventType::Auth => "Auth".to_string(),
            EventType::Other => "Other".to_string(),
        }
    }
}
