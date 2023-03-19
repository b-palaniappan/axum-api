use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct CreateUser {
  #[validate(length(min = 2))]
  #[serde(rename = "firstName")]
  pub first_name: String,
  #[serde(rename = "lastName")]
  pub last_name: String,
  #[validate(email)]
  pub email: String,
  #[serde(rename = "addressLineOne")]
  pub address_line_one: String,
  #[serde(rename = "addressLineTwo")]
  pub address_line_tow: Option<String>,
  pub city: String,
  pub state: String,
  pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredUser {
  pub id: String,
  #[serde(rename = "firstName")]
  pub first_name: String,
  #[serde(rename = "lastName")]
  pub last_name: String,
  pub email: String,
  #[serde(rename = "addressLineOne")]
  pub address_line_one: String,
  #[serde(rename = "addressLineTwo")]
  pub address_line_tow: Option<String>,
  pub city: String,
  pub state: String,
  pub country: String,
}
