use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize,
    pub iat: usize,
}

impl Default for Claims {
    fn default() -> Self {
        let now = Utc::now();
        let expire: Duration = Duration::hours(24);
        let exp: usize = (now + expire).timestamp() as usize;
        let iat: usize = now.timestamp() as usize;
        Claims {
            user_id: String::new(),
            exp,
            iat,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthBody {
    pub access_token: String,
    pub refresh_token: String,
}
