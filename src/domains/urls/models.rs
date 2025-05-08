use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub url: String,
    pub short_url: String,
    pub favourite: bool,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
}
