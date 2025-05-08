use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UrlRequest {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UrlResponse {
    pub id: String,
    pub url: String,
    pub short_url: String,
    pub favourite: bool,
    pub deleted: bool,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct FavouriteUrl {
    pub favourite: bool,
}
