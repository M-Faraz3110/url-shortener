use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{common::errors::AppError, config::config::Config};

use super::models::Claims;

#[derive(Clone)]
pub struct Auth {
    pub private_key_enc: EncodingKey,
    //pub private_key_dec: DecodingKey,
}

impl Auth {
    pub fn new(config: &Config) -> Self {
        let private_key_base64 = config.jwt_private_key.clone();
        let private_key =
            base64::decode(private_key_base64).expect("Failed to decode JWT private key");
        Auth {
            private_key_enc: EncodingKey::from_secret(&private_key),
            //private_key_dec: DecodingKey::from_secret(&private_key),
        }
    }

    pub fn generate_jwt_token(&self, user_id: &String) -> Result<String, AppError> {
        let claims = Claims {
            user_id: user_id.clone(),
            ..Default::default()
        };
        encode(&Header::default(), &claims, &self.private_key_enc)
            .map_err(|_| AppError::TokenCreation)
    }
}
