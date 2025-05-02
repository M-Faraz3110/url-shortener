use std::sync::Arc;

use axum::Error;
use regex::Regex;
use uuid::Uuid;

use crate::{
    common::{constants, errors::AppError},
    config::config::Config,
    users::{interface::UserRepository, repository::UsersRepo},
};

use super::{dto::UrlResponse, interface::UrlRepository, repository::UrlRepo};

#[derive(Clone)]
pub struct UrlService {
    url_repo: Arc<dyn UrlRepository + Send + Sync>,
    user_repo: Arc<dyn UserRepository + Send + Sync>,
    prefix: String,
}

impl UrlService {
    pub fn new(url_repo: UrlRepo, user_repo: Arc<UsersRepo>, config: &Config) -> Self {
        UrlService {
            url_repo: Arc::new(url_repo),
            user_repo: user_repo,
            prefix: config.service_host.clone(),
        }
    }

    pub async fn shorten_url(
        &self,
        url: &String,
        user_id: &String,
    ) -> Result<UrlResponse, AppError> {
        //check for safety of the URL
        //<CODE>
        let re = Regex::new(constants::URL_REGEX)
            .map_err(|_| AppError::ValidationError("Regex Error".to_string()))?;
        if !re.is_match(url) {
            return Err(AppError::ValidationError("Invalid URL".to_string()));
        }

        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .await
            .map_err(|_| AppError::NotFound("User not found".to_string()))?;

        let hash = format!("{:x}", md5::compute(url));
        let short_url = &hash[..8].to_string();
        match self.url_repo.create(url, short_url, &user.id).await {
            Ok(url) => {
                return Ok(UrlResponse {
                    id: url.id.to_string(),
                    url: url.url,
                    short_url: format!("{}/{}", self.prefix, url.short_url),
                    deleted: false,
                    created_at: url.created_at.to_string(),
                });
            }
            Err(e) => {
                return Err(AppError::DatabaseError(e));
            }
        }
    }

    pub async fn delete_url(&self, id: &String) -> Result<UrlResponse, AppError> {
        match self.url_repo.delete(id).await {
            Ok(url) => {
                return Ok(UrlResponse {
                    id: url.id.to_string(),
                    url: url.url,
                    short_url: url.short_url,
                    deleted: true,
                    created_at: url.created_at.to_string(),
                });
            }
            Err(e) => Err(AppError::DatabaseError(e)),
        }
    }

    pub async fn enter_url(&self, short_url: &String) -> Result<UrlResponse, AppError> {
        match self.url_repo.get_url_by_short_url(short_url).await {
            Ok(url) => {
                return Ok(UrlResponse {
                    id: url.id.to_string(),
                    url: url.url,
                    short_url: url.short_url,
                    deleted: false,
                    created_at: url.created_at.to_string(),
                });
            }
            Err(e) => Err(AppError::DatabaseError(e)),
        }
    }
}
