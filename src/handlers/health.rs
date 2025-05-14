use axum::{
    Extension, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get,
};
use utoipa::OpenApi;

use crate::{
    app_state::AppState,
    common::{errors::AppError, response::ApiResponse},
    domains::auth::models::Claims,
};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check"),
    ),
)]
#[axum::debug_handler]
pub async fn health_check(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    println!("Health check hit!");
    Ok(ApiResponse::success(StatusCode::OK, "SUCCESS"))
    //
}

#[derive(OpenApi)]
#[openapi(
    paths(health_check),
    components(),
    tags(
        (name = "Health")
    ),
    modifiers(&HealthApiDoc)
)]
pub struct HealthApiDoc;

impl utoipa::Modify for HealthApiDoc {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
    }
}

pub fn health_route() -> Router<AppState> {
    Router::new().route("/", get(health_check))
}
