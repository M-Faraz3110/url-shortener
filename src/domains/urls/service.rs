use std::sync::Arc;

use axum::Error;
use regex::Regex;
use url::Url;
use uuid::Uuid;

use crate::{
    common::{constants, errors::AppError},
    config::config::Config,
    infra::repositories::{
        urls::{interface::UrlRepository, repository::UrlRepo},
        users::{interface::UserRepository, repository::UsersRepo},
    },
};

use super::dto::UrlResponse;

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
        let parsed = Url::parse(url)
            .map_err(|_| AppError::ValidationError("URL Error".to_string()))?
            .to_string();

        let user = self
            .user_repo
            .get_user_by_id(user_id)
            .await
            .map_err(|_| AppError::NotFound("User not found".to_string()))?;

        let hash = format!("{:x}", md5::compute(parsed.clone()));
        let short_url = &hash[..8].to_string();
        match self.url_repo.create(&parsed, short_url, &user.id).await {
            Ok(url) => {
                return Ok(UrlResponse {
                    id: url.id.to_string(),
                    url: url.url,
                    short_url: format!("{}/{}", self.prefix, url.short_url),
                    favourite: url.favourite,
                    deleted: false,
                    created_at: url.created_at.to_string(),
                });
            }
            Err(e) => {
                println!("{:?}", e);
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
                    favourite: url.favourite,
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
                    favourite: url.favourite,
                    deleted: false,
                    created_at: url.created_at.to_string(),
                });
            }
            Err(e) => Err(AppError::DatabaseError(e)),
        }
    }

    pub async fn favourite_url(&self, id: &String, state: &bool) -> Result<UrlResponse, AppError> {
        match self.url_repo.favourite_url(id, state).await {
            Ok(url) => {
                return Ok(UrlResponse {
                    id: url.id.to_string(),
                    url: url.url,
                    short_url: url.short_url,
                    favourite: url.favourite,
                    deleted: false,
                    created_at: url.created_at.to_string(),
                });
            }
            Err(e) => {
                println!("{:?}", e);
                Err(AppError::DatabaseError(e))
            }
        }
    }

    pub async fn get_user_urls(&self, user_id: &String) -> Result<Vec<UrlResponse>, AppError> {
        match self.url_repo.get_user_urls(user_id).await {
            Ok(urls) => {
                let mut url_responses = Vec::new();
                for url in urls {
                    url_responses.push(UrlResponse {
                        id: url.id.to_string(),
                        url: url.url,
                        short_url: format!("{}/{}", self.prefix, url.short_url),
                        favourite: url.favourite,
                        deleted: false,
                        created_at: url.created_at.to_string(),
                    });
                }
                return Ok(url_responses);
            }
            Err(e) => Err(AppError::DatabaseError(e)),
        }
    }
}
