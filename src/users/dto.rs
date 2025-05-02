use chrono::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Register {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    // pub expiry: DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
