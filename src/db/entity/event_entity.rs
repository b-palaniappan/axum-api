use chrono::{DateTime, Utc};

pub struct Event {
  pub id: i64,
  pub event_type: EventType,
  pub event_name: EventName,
  pub event_time: DateTime<Utc>,
  pub user_id: i64,
}

pub enum EventType {
  User,
  Auth,
  Other
}

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
