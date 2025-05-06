use super::dto::UrlRequest;
use crate::auth::models::Claims;
use crate::common::response::ApiResponse;
use crate::urls::dto::UrlResponse;
use crate::{app_state::AppState, common::errors::AppError};
use axum::response::Redirect;
use axum::{
    Extension, Json,
    extract::{Path, State},
    response::IntoResponse,
};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/urls/shorten",
    request_body = UrlRequest,
    responses(
        (status = 200, description = "URL shortened successfully", body = UrlResponse),
    ),
)]
#[axum::debug_handler]
pub async fn shorten_url(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UrlRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut payload = payload;
    println!("Claims: {:?}", claims);
    let url = state
        .url_service
        .shorten_url(&payload.url, &claims.user_id)
        .await?;
    Ok(ApiResponse::success(url))
    //
}

#[utoipa::path(
    delete,
    path = "/urls/delete/{id}",
    responses((status = 200, description = "url deleted", body = UrlResponse)),
)]
#[axum::debug_handler]
pub async fn delete_url(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let url = state.url_service.delete_url(&id).await?;
    Ok(ApiResponse::success(url))
}

#[utoipa::path(
    get,
    path = "/{code}",
    responses((status = 302, description = "url redirect")),
)]
#[axum::debug_handler]
pub async fn enter_url(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let url = state.url_service.enter_url(&code).await?;
    Ok(Redirect::permanent(url.url.as_str()))
}

#[utoipa::path(
    post,
    path = "/urls/favourite/{id}",
    responses((status = 200, description = "url favourited", body = UrlResponse)),
)]
#[axum::debug_handler]
pub async fn toggle_favourite_url(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let url = state.url_service.favourite_url(&id).await?;
    Ok(ApiResponse::success(url))
}
