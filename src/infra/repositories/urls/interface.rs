use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

use async_trait::async_trait;

use crate::domains::urls::models::Url;

#[async_trait]
pub trait UrlRepository: Send + Sync {
    async fn create(
        &self,
        url_req: &String,
        short_url: &String,
        user_id: &Uuid,
    ) -> Result<Url, sqlx::Error>;

    async fn delete(&self, id: &String) -> Result<Url, sqlx::Error>;

    async fn get_url_by_short_url(&self, short_url: &String) -> Result<Url, sqlx::Error>;

    async fn favourite_url(&self, id: &String, state: &bool) -> Result<Url, sqlx::Error>;

    async fn get_user_urls(&self, user_id: &String) -> Result<Vec<Url>, sqlx::Error>;
}
