use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::{interface::UserRepository, models::User};

pub struct UsersRepo {
    db: Pool<Postgres>,
}

impl UsersRepo {
    pub fn new(db: Pool<Postgres>) -> Self {
        UsersRepo { db }
    }
}

#[async_trait]
impl UserRepository for UsersRepo {
    async fn register(&self, username: &String, password: &String) -> Result<(), sqlx::Error> {
        let mut tx = self.db.begin().await?;
        let id = uuid::Uuid::new_v4();
        let _user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, password_hash, deleted, created_at)
            VALUES ($1, $2, $3, false, NOW())
            RETURNING id, username, password_hash, deleted, created_at
            "#,
            id,
            username,
            password
        )
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;

        Ok(())
    }

    async fn login(&self, username: &String, password: &String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, deleted, created_at
            FROM users
            WHERE username = $1 AND password_hash = $2 AND deleted = false
            "#,
            username,
            password
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    async fn get_user_by_username(&self, username: &String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, deleted, created_at
            FROM users
            WHERE username = $1 AND deleted = false
            "#,
            username
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    async fn get_user_by_id(&self, user_id: &String) -> Result<User, sqlx::Error> {
        let uuid_id = uuid::Uuid::parse_str(user_id)
            .map_err(|_| sqlx::Error::Decode("Invalid UUID".into()))?;
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, deleted, created_at
            FROM users
            WHERE id = $1 AND deleted = false
            "#,
            uuid_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }
}
