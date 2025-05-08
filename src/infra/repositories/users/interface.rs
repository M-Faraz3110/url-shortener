use async_trait::async_trait;

use crate::domains::users::models::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn register(&self, username: &String, password: &String) -> Result<(), sqlx::Error>;

    async fn login(&self, username: &String, password: &String) -> Result<User, sqlx::Error>;

    async fn get_user_by_username(&self, username: &String) -> Result<User, sqlx::Error>;

    async fn get_user_by_id(&self, id: &String) -> Result<User, sqlx::Error>;
}
