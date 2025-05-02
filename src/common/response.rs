use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: u16,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: 200,
            data: Some(data),
        }
    }

    pub fn failure(status: u16, data: T) -> Self {
        ApiResponse {
            status: status,
            data: Some(data),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
