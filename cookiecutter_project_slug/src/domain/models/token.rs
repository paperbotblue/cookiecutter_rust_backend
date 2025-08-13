use super::role::ClientType;
use actix_web::{FromRequest, HttpMessage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Token {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_type: ClientType,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateToken {
    pub client_id: Uuid,
    pub client_type: ClientType,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UpdateToken {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_type: ClientType,
    pub is_revoked: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub sub_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: Uuid,
    pub permissions: String,
}

impl FromRequest for JwtClaims {
    type Error = actix_web::Error;
    type Future = futures_util::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // Extract claims from request extensions (assuming your middleware puts them there)
        if let Some(claims) = req.extensions().get::<JwtClaims>() {
            futures_util::future::ready(Ok(claims.clone()))
        } else {
            futures_util::future::ready(Err(actix_web::error::ErrorUnauthorized(
                "Missing or invalid JWT",
            )))
        }
    }
}
