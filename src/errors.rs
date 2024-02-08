use std::num::TryFromIntError;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InternalServerError,
    UnprocessableEntity,
}

impl ApiError {
    pub fn not_found() -> Self {
        ApiError::NotFound
    }

    pub fn internal_server_error() -> Self {
        ApiError::InternalServerError
    }

    pub fn unprocessable_entity() -> Self {
        ApiError::UnprocessableEntity
    }
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound => {
                write!(f, "NOT_FOUND")
            }
            ApiError::InternalServerError => {
                write!(f, "INTERNAL_SERVER_ERROR")
            }
            ApiError::UnprocessableEntity => {
                write!(f, "UNPROCESSABLE_ENTITY")
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND).into_response(),
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            ApiError::UnprocessableEntity => (StatusCode::UNPROCESSABLE_ENTITY).into_response(),
        }
    }
}

impl From<TryFromIntError> for ApiError {
    fn from(_: TryFromIntError) -> Self {
        ApiError::internal_server_error()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(_: sqlx::Error) -> Self {
        ApiError::internal_server_error()
    }
}
