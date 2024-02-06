use axum::{http::StatusCode, response::IntoResponse};

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
                (StatusCode::NOT_FOUND, msg.unwrap_or_default()).into_response()
            }
            ApiError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.unwrap_or_default()).into_response()
            }
            ApiError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg.unwrap_or_default()).into_response()
            }
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(_: sqlx::Error) -> Self {
        ApiError::InternalServerError(None)
    }
}
