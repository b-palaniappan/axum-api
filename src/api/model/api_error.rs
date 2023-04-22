use serde::Serialize;

// Model for error handling.
#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub status: u16,
    pub time: String,
    pub message: String,
    #[serde(rename = "debugMessage")]
    pub debug_message: Option<String>,
    #[serde(rename = "subErrors")]
    pub sub_errors: Vec<ValidationError>,
}

// Sub errors for validation error
#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub object: String,
    pub field: String,
    pub rejected_value: Option<String>,
    pub message: String,
}
