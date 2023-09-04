use std::borrow::Cow;

use axum::{http::StatusCode, Json, response::IntoResponse};
use axum::response::Response;
use chrono::{SecondsFormat, Utc};
use derive_more::{Display, Error};
use serde::Serialize;
use serde_json::json;
use tracing::info;
use validator::ValidationErrors;

// -- Error handing.
#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "Internal server error. Try again after some time.")]
    InternalServerError,

    #[display(fmt = "Resource not found")]
    ResourceNotFound,

    #[display(fmt = "Bad Request")]
    BadRequest,

    #[display(fmt = "Validation error on field")]
    ValidationError {
        validation_error: ValidationErrors,
        object: String,
    },
}

#[derive(Debug, Serialize)]
pub struct ValidationError {
    object: String,
    field: String,
    rejected_value: String,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub status: u16,
    pub time: String,
    pub message: String,
    pub debug_message: Option<String>,
    pub sub_errors: Vec<ValidationError>,
}

impl IntoResponse for AppError {
    // Global error handler Http Response payload
    fn into_response(self) -> Response {
        let (status, api_error) = match &self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
                    message: self.to_string(),
                    debug_message: None,
                    sub_errors: vec![],
                },
            ),
            Self::ValidationError {
                validation_error,
                object,
            } => {
                let mut validation_sub_errs = vec![];
                // Iterate thru validation error object
                for (field, field_errors) in validation_error.field_errors() {
                    for field_error in field_errors {
                        info!("Field Error --> {:?}", field_error);
                        validation_sub_errs.push(ValidationError {
                            object: object.to_string(),
                            field: field.to_owned(),
                            rejected_value: field_error.params.get("value").unwrap().to_string(),
                            message: field_error
                                .message
                                .as_ref()
                                .unwrap_or_else(|| &Cow::Borrowed(""))
                                .to_string(),
                        })
                    }
                }

                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    ApiError {
                        status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                        time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
                        message: self.to_string(),
                        debug_message: None,
                        sub_errors: validation_sub_errs,
                    },
                )
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
                    message: self.to_string(),
                    debug_message: None,
                    sub_errors: vec![],
                },
            ),
        };
        (status, Json(json!(api_error))).into_response()
    }
}
