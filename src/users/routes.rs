use axum::{Router, routing::delete, routing::post};
use utoipa::OpenApi;

use crate::{
    app_state::AppState,
    users::dto::{LoginRequest, LoginResponse, Register},
};

use super::handlers;
use crate::users::handlers::{login, register};

#[derive(OpenApi)]
#[openapi(
    paths(handlers::register, handlers::login),
    components(schemas(Register, LoginResponse, LoginRequest)),
    tags(
        (name = "Users", description = "Operations related to users")
    ),
    modifiers(&UserApiDoc)
)]
pub struct UserApiDoc;

impl utoipa::Modify for UserApiDoc {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
    }
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
}
