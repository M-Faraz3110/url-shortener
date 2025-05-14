use axum::{Router, middleware};
use jsonwebtoken::DecodingKey;
use utoipa::OpenApi;

use crate::{
    app_state::AppState,
    config::config::Config,
    handlers::{
        health::{HealthApiDoc, health_route},
        urls::{UrlApiDoc, url_routes},
        users::{UserApiDoc, user_routes},
    },
    middleware::jwt::jwt_auth,
};
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router(config: &Config, state: AppState) -> Router {
    let cors = tower_http::cors::CorsLayer::permissive();
    let public_routes = Router::new()
        .nest("/users", user_routes())
        .nest("/health", health_route())
        .layer(cors.clone());

    let private_routes = Router::new()
        .merge(url_routes())
        .route_layer(middleware::from_fn(jwt_auth))
        .layer(cors);

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
        .url("/api-docs/health/openapi.json", HealthApiDoc::openapi())
}
