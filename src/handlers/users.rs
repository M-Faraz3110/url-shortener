use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use utoipa::OpenApi;

use crate::{
    app_state::AppState,
    common::{errors::AppError, response::ApiResponse},
    domains::users::dto::{LoginRequest, LoginResponse, Register},
};

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/users/login",
    responses((status = 200, description = "login user", body = LoginResponse)),
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut payload = payload;

    let user = state
        .user_service
        .login(&payload.username, &payload.password)
        .await?;
    Ok(ApiResponse::success(StatusCode::OK, user))
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/users/register",
    responses((status = 200, description = "register user")),
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<Register>,
) -> Result<impl IntoResponse, AppError> {
    let mut payload = payload;

    let user = state
        .user_service
        .register(&payload.username, &payload.password)
        .await?;
    Ok(ApiResponse::success(StatusCode::OK, user)) //empty
}

#[derive(OpenApi)]
#[openapi(
    paths(register, login),
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
        .route("/register", post(register))
        .route("/login", post(login))
}
