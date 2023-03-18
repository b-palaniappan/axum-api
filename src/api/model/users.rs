use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub address_line_one: String,
  pub address_line_tow: Option<String>,
  pub city: String,
  pub state: String,
  pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredUser {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub address_line_one: String,
  pub address_line_tow: Option<String>,
  pub city: String,
  pub state: String,
  pub country: String,
}
