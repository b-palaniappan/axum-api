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
  #[validate(length(equal = 2))]
  pub state: String,
  pub country: String,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UpdateUser {
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
  #[validate(length(equal = 2))]
  pub state: String,
  pub country: String,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct PatchUser {
  #[validate(length(min = 2))]
  #[serde(rename = "firstName")]
  pub first_name: Option<String>,
  #[serde(rename = "lastName")]
  pub last_name: Option<String>,
  #[validate(email)]
  pub email: Option<String>,
  #[serde(rename = "addressLineOne")]
  pub address_line_one: Option<String>,
  #[serde(rename = "addressLineTwo")]
  pub address_line_tow: Option<String>,
  pub city: Option<String>,
  #[validate(length(equal = 2))]
  pub state: Option<String>,
  pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredUser {
  pub id: i64,
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
