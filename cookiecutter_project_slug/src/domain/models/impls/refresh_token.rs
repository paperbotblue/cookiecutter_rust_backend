use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::domain::models::refresh_token::{CreateRefreshToken, JwtClaims};

impl CreateRefreshToken {
    pub fn new(user_id: Uuid, token: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::days(30);
        Self {
            user_id,
            token,
            family_id: Uuid::new_v4(),
            issued_at: iat,
            expires_at: exp,
            is_revoked: false,
        }
    }
}

impl JwtClaims {
    pub fn new(user_id: Uuid, role: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::minutes(15);
        Self {
            sub: user_id.to_string(),
            role,
            exp: exp.timestamp() as usize,
            iat: iat.timestamp() as usize,
        }
    }
}
