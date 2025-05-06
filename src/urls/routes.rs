use crate::app_state::AppState;
use crate::urls::dto::{UrlRequest, UrlResponse};
use axum::routing::get;
use axum::{Router, routing::delete, routing::post};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use super::handlers;
use utoipa::OpenApi;

use crate::urls::handlers::{delete_url, shorten_url};

#[derive(OpenApi)]
#[openapi(
    paths(handlers::shorten_url, handlers::delete_url),
    components(schemas(UrlRequest, UrlResponse)),
    tags(
        (name = "URLs", description = "Operations related to URL shortening")
    ),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&UrlApiDoc)
)]
pub struct UrlApiDoc;
impl utoipa::Modify for UrlApiDoc {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("Input your `<your‑jwt>`"))
                    .build(),
            ),
        )
    }
}

pub fn url_routes() -> Router<AppState> {
    Router::new()
        .route("/urls/shorten", post(handlers::shorten_url))
        .route("/urls/delete/{id}", delete(handlers::delete_url))
        .route("/{code}", get(handlers::enter_url))
        .route("/urls/favourite/{id}", post(handlers::toggle_favourite_url))
}
