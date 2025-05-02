use axum::{
    BoxError,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use thiserror::Error;
use tracing::error;

use super::response::ApiResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError), // Used for database-related errors

    // #[error("Hashing error: {0}")]
    // HashingError(#[from] argon2::Error), // Used for hashing-related errors
    #[error("Hashing error: {0}")]
    PasswordHashingError(argon2::password_hash::Error),

    #[error("Not found: {0}")]
    NotFound(String), // Used for not found errors

    #[error("Internal server error")]
    InternalError,

    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Used for authentication-related errors
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token creation error")]
    TokenCreation,
    #[error("User not found")]
    UserNotFound,
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::PasswordHashingError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AppError::MissingCredentials => StatusCode::BAD_REQUEST,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UserNotFound => StatusCode::NOT_FOUND,
        };
        let body = axum::Json(ApiResponse::<Option<String>>::failure(
            status.as_u16(),
            Some(self.to_string()),
        ));

        (status, body).into_response()
    }
}

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    let status = if error.is::<tower::timeout::error::Elapsed>() {
        StatusCode::REQUEST_TIMEOUT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    let message = error.to_string();
    error!(?status, %message, "Request failed");

    let body = ApiResponse::<Option<String>>::failure(status.as_u16(), Some(message));

    (status, body)
}
