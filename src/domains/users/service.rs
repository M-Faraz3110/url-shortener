use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::DateTime;

use crate::{
    common::errors::AppError,
    domains::auth::auth::Auth,
    infra::repositories::users::{interface::UserRepository, repository::UsersRepo},
};

use super::dto::LoginResponse;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
    authrepo: Auth,
}

impl UserService {
    pub fn new(repo: Arc<UsersRepo>, authrepo: Auth) -> Self {
        Self {
            repo: repo,
            authrepo: authrepo,
        }
    }

    pub async fn login(
        &self,
        username: &String,
        password: &String,
    ) -> Result<LoginResponse, AppError> {
        if username.is_empty() || password.is_empty() {
            return Err(AppError::ValidationError(
                "Username and password cannot be empty".to_string(),
            ));
        }

        let user = self
            .repo
            .get_user_by_username(username)
            .await
            .map_err(|_| AppError::NotFound("User not found".to_string()))?;

        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&user.password_hash)?;

        if argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            let user_id = user.id.to_string();
            let token_resp = self
                .authrepo
                .generate_jwt_token(&user_id)
                .map_err(|_| AppError::InternalError)?;
            let login_response = LoginResponse { token: token_resp };
            Ok(login_response)
        } else {
            return Err(AppError::ValidationError("Invalid credentials".to_string()));
        }
    }

    pub async fn register(&self, username: &String, password: &String) -> Result<(), AppError> {
        if username.is_empty() || password.is_empty() {
            return Err(AppError::ValidationError(
                "Username and password cannot be empty".to_string(),
            ));
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        match self.repo.register(username, &password_hash).await {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::DatabaseError(e)),
        }
    }
}
