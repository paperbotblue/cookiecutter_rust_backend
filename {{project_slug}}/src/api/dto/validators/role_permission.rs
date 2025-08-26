use std::sync::LazyLock;

use regex::Regex;
use serde::de::Error as DeError;
use serde::Deserializer;

static DESCRIPTION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]+$").unwrap());

pub fn validate_role_permission_fields<'de, D: Deserializer<'de>>(
    description: &str,
) -> Result<(), D::Error> {
    if description.len() < 3 || description.len() > 2048 {
        return Err(D::Error::custom(
            "Description must be between 3 and 2048 characters",
        ));
    }

    if !DESCRIPTION_RE.is_match(description) {
        return Err(D::Error::custom(
            "Description can only contain letters and numbers",
        ));
    }

    Ok(())
}
