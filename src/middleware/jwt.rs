use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{common::errors::AppError, config::config::Config, domains::auth::models::Claims};

pub async fn jwt_auth<B>(mut req: Request<B>, next: Next) -> Result<Response, Response>
where
    B: Send + Into<axum::body::Body>,
{
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .ok_or_else(|| AppError::InvalidToken.into_response())?;

    let private_key = get_private_key().await;

    let token_data =
        decode::<Claims>(token, &private_key, &Validation::default()).map_err(|err| {
            tracing::error!("Error decoding token: {:?}", err);
            AppError::InvalidToken.into_response()
        })?;

    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req.map(Into::into)).await)
}

pub async fn get_private_key() -> DecodingKey {
    let config = Config::from_env();
    let private_key_base64 = config.unwrap().jwt_private_key.clone();
    let private_key = base64::decode(private_key_base64).expect("Failed to decode JWT private key");
    DecodingKey::from_secret(&private_key)
}
