use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Address {
  pub id: i64,
  pub user_id: i64,
  pub line_one: String,
  pub line_two: String,
  pub city: String,
  pub state: String,
  pub country: String,
  pub geo_code: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub deleted_at: DateTime<Utc>
}
