use crate::common::response::ApiResponse;
use crate::domains::auth::models::Claims;
use crate::domains::urls::dto::{FavouriteUrl, UrlRequest, UrlResponse};
use crate::{app_state::AppState, common::errors::AppError};
use axum::Router;
use axum::http::StatusCode;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::response::Redirect;
use axum::{
    Extension, Json,
    extract::{Path, State},
    response::IntoResponse,
    routing::delete,
    routing::get,
    routing::patch,
    routing::post,
};
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
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
    Ok(ApiResponse::success(StatusCode::OK, url))
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
    Ok(ApiResponse::success(StatusCode::OK, url))
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
    patch,
    path = "/urls/favourite/{id}",
    responses((status = 200, description = "url favourited", body = UrlResponse)),
)]
#[axum::debug_handler]
pub async fn favourite_url(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<FavouriteUrl>,
) -> Result<impl IntoResponse, AppError> {
    let url = state
        .url_service
        .favourite_url(&id, &payload.favourite)
        .await?;
    Ok(ApiResponse::success(StatusCode::OK, url))
}

#[utoipa::path(
    get,
    path = "/urls/user",
    responses((status = 200, description = "user urls", body = Vec<UrlResponse>)),
)]
#[axum::debug_handler]
pub async fn get_user_urls(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let urls = state.url_service.get_user_urls(&claims.user_id).await?;
    Ok(ApiResponse::success(StatusCode::OK, urls))
}

#[derive(OpenApi)]
#[openapi(
    paths(shorten_url, delete_url, enter_url, favourite_url, get_user_urls),
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
    let cors = tower_http::cors::CorsLayer::permissive()
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_origin(tower_http::cors::Any);

    Router::new()
        .route("/urls/shorten", post(shorten_url))
        .route("/urls/delete/{id}", delete(delete_url))
        .route("/{code}", get(enter_url))
        .route("/urls/favourite/{id}", patch(favourite_url))
        .route("/urls/user", get(get_user_urls))
        .layer(cors)
}
