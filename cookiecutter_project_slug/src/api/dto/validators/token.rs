use std::sync::LazyLock;

use regex::Regex;
use serde::de::Error as DeError;
use serde::Deserializer;

use crate::domain::models::role::ClientType;

static TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*$").unwrap());

pub fn validate_token_fields<'de, D: Deserializer<'de>>(
    token: &str,
    client_type: &ClientType,
) -> Result<(), D::Error> {
    if token.len() < 10 || token.len() > 255 {
        return Err(D::Error::custom("Invalid token"));
    }

    if *client_type == ClientType::Invalid {
        return Err(D::Error::custom("invalid client type"));
    }

    if !TOKEN_RE.is_match(token) {
        return Err(D::Error::custom("Invalid Token"));
    }
    Ok(())
}
