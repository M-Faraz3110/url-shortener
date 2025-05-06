use async_trait::async_trait;
use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

use super::{dto::UrlRequest, interface::UrlRepository, models::Url};

pub struct UrlRepo {
    db: Pool<Postgres>,
}

impl UrlRepo {
    pub fn new(db: Pool<Postgres>) -> Self {
        UrlRepo { db }
    }
}

#[async_trait]
impl UrlRepository for UrlRepo {
    async fn create(
        &self,
        //tx: &mut Transaction<'_, Postgres>,
        url_req: &String,
        short_url: &String,
        user_id: &Uuid,
    ) -> Result<Url, sqlx::Error> {
        let mut tx = self.db.begin().await?;
        let id = Uuid::new_v4();
        let url = sqlx::query_as!(
            Url,
            r#"
            INSERT INTO urls (id, user_id, url, short_url, deleted, created_at)
            VALUES ($1, $2, $3, $4, false, NOW())
            RETURNING id, user_id, url, short_url, favourite, deleted, created_at
            "#,
            id,
            user_id,
            url_req,
            short_url,
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;

        Ok(url)
    }

    async fn delete(&self, id: &String) -> Result<Url, sqlx::Error> {
        let mut tx = self.db.begin().await?;
        let uuid_id =
            Uuid::parse_str(id).map_err(|_| sqlx::Error::Decode("Invalid UUID".into()))?;

        let url = sqlx::query_as!(
            Url,
            r#"
            UPDATE urls
            SET deleted = true
            WHERE id = $1
            RETURNING id, user_id, url, short_url, favourite, deleted, created_at
            "#,
            uuid_id,
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;

        Ok(url)
    }

    async fn get_url_by_short_url(&self, short_url: &String) -> Result<Url, sqlx::Error> {
        let url = sqlx::query_as!(
            Url,
            r#"
            SELECT id, user_id, url, short_url, favourite, deleted, created_at
            FROM urls
            WHERE short_url = $1 AND deleted = false
            "#,
            short_url
        )
        .fetch_one(&self.db)
        .await?;

        Ok(url)
    }

    async fn favourite_url(&self, id: &String) -> Result<Url, sqlx::Error> {
        let mut tx = self.db.begin().await?;
        let uuid_id =
            Uuid::parse_str(id).map_err(|_| sqlx::Error::Decode("Invalid UUID".into()))?;

        let url = sqlx::query_as!(
            Url,
            r#"
            UPDATE urls
            SET favourite = NOT favourite
            WHERE id = $1
            RETURNING id, user_id, url, short_url, favourite, deleted, created_at
            "#,
            uuid_id,
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;

        Ok(url)
    }
}
