use std::num::TryFromIntError;

use axum::{http::StatusCode, response::IntoResponse};
use tracing::error;

#[derive(Debug)]
pub enum ApiError {
    NotFound(Option<String>),
    InternalServerError(Option<String>),
    BadRequest(Option<String>),
}

impl ApiError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        ApiError::NotFound(Some(msg.into()))
    }

    pub fn internal_server_error(msg: impl Into<String>) -> Self {
        ApiError::InternalServerError(Some(msg.into()))
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        ApiError::BadRequest(Some(msg.into()))
    }
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound(msg) => {
                write!(f, "NOT_FOUND: {}", msg.as_deref().unwrap_or(""))
            }
            ApiError::InternalServerError(msg) => {
                write!(f, "INTERNAL_SERVER_ERROR: {}", msg.as_deref().unwrap_or(""))
            }
            ApiError::BadRequest(msg) => {
                write!(f, "BAD_REQUEST: {}", msg.as_deref().unwrap_or(""))
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound(msg) => {
                let msg = msg.unwrap_or_default();
                error!("NOT_FOUND: {}", msg);
                (StatusCode::NOT_FOUND, msg).into_response()
            }
            ApiError::InternalServerError(msg) => {
                let msg = msg.unwrap_or_default();
                error!("INTERNAL_SERVER_ERROR: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            ApiError::BadRequest(msg) => {
                let msg = msg.unwrap_or_default();
                error!("BAD_REQUEST: {}", msg);
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
        }
    }
}

impl From<TryFromIntError> for ApiError {
    fn from(_: TryFromIntError) -> Self {
        ApiError::InternalServerError(Some("Conversion overflow".to_string()))
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(_: sqlx::Error) -> Self {
        ApiError::InternalServerError(None)
    }
}
