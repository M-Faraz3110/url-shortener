use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(status: StatusCode, data: T) -> Self {
        ApiResponse {
            status: status,
            data: Some(data),
        }
    }

    pub fn failure(status: StatusCode, data: T) -> Self {
        ApiResponse {
            status: status,
            data: Some(data),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (self.status, axum::Json(self.data)).into_response()
    }
}
