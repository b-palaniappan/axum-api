use chrono::{DateTime, Utc};

pub struct User {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub deleted_at: DateTime<Utc>,
}
