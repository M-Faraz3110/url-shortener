use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    app_state::AppState,
    common::{errors::AppError, response::ApiResponse},
    users::dto::LoginResponse,
};

use super::dto::{LoginRequest, Register};

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
    Ok(ApiResponse::success(user))
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
    Ok(ApiResponse::success(user)) //empty
}
