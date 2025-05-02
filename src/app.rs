use axum::{Router, middleware};
use jsonwebtoken::DecodingKey;
use utoipa::OpenApi;

use crate::{
    app_state::AppState,
    auth,
    config::config::Config,
    middleware::jwt::jwt_auth,
    urls::routes::{UrlApiDoc, url_routes},
    users::routes::{UserApiDoc, user_routes},
};
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router(config: &Config, state: AppState) -> Router {
    //let state = AppState::new();
    let public_routes = Router::new().nest("/users", user_routes());
    //Router::new().nest("/api/v1", public_routes);
    //let private_key_base64 = config.jwt_private_key.clone();
    //let private_key = base64::decode(private_key_base64).expect("Failed to decode JWT private key");
    //let private_key_dec = DecodingKey::from_secret(&private_key);

    let private_routes = Router::new()
        .nest("/urls", url_routes())
        .route_layer(middleware::from_fn(jwt_auth));

    Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .merge(create_swagger_ui())
        .with_state(state)
}

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

pub fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/urls/openapi.json", UrlApiDoc::openapi())
        .url("/api-docs/users/openapi.json", UserApiDoc::openapi())
}
