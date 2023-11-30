use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub message: String,
    #[serde(skip_serializing)]
    pub code: u16,
}

impl ApiError {
    pub fn new(message: &str, code: u16) -> ApiError {
        ApiError {
            message: message.to_string(),
            code,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        // TODO: log error
        match error {
            DieselError::NotFound => ApiError::new("Record not found", 404),
            _ => ApiError::new("Internal server error", 500),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.code).unwrap()).json(self)
    }
}
