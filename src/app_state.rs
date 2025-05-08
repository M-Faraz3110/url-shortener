use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::{
    config::config::Config,
    domains::{auth::auth::Auth, urls::service::UrlService, users::service::UserService},
    infra::repositories::{urls::repository::UrlRepo, users::repository::UsersRepo},
};

#[derive(Clone)]
pub struct AppState {
    pub url_service: UrlService,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(config: &Config, pool: Pool<Postgres>) -> Self {
        let url_repo = UrlRepo::new(pool.clone());
        let users_repo = Arc::new(UsersRepo::new(pool.clone()));

        let url_service = UrlService::new(url_repo, Arc::clone(&users_repo), config);
        let auth_repo = Auth::new(config);
        let user_service = UserService::new(Arc::clone(&users_repo), auth_repo);

        Self {
            url_service,
            user_service,
        }
    }
}
