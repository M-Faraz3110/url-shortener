use axum::{
    BoxError,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::{Deserialize, Serialize};
use sqlx::{Error as SqlxError, error::DatabaseError};
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

    #[error("Invalid token")]
    InvalidToken,
    #[error("Token creation error")]
    TokenCreation,
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::PasswordHashingError(err)
    }
}

// impl From<SqlxError> for AppError {
//     fn from(err: SqlxError) -> Self {
//         match err.as_database_error() {
//             Some(er) => {
//                 if er.is_unique_violation() {
//                     return AppError::AlreadyExistsError(err);
//                 }
//                 return AppError::DatabaseError(err);
//             }
//             None => {
//                 return AppError::InternalError;
//             }
//         };
//     }
// }

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let err = &self;
        let status = match err {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(err) => {
                let sql_err = err.as_database_error();
                match sql_err {
                    Some(er) => {
                        if er.is_unique_violation() {
                            StatusCode::CONFLICT
                        } else {
                            StatusCode::INTERNAL_SERVER_ERROR
                        }
                    }
                    None => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
            AppError::PasswordHashingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = ApiResponse::failure(status, self.to_string());

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

    let body = ApiResponse::failure(status, message);

    (status, body)
}
