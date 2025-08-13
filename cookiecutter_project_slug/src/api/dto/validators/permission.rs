use std::sync::LazyLock;

use regex::Regex;
use serde::de::Error as DeError;
use serde::Deserializer;

static NAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").expect(""));
static DESCRIPTION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]+$").unwrap());

pub fn validate_permission_fields<'de, D: Deserializer<'de>>(
    name: &str,
    description: &str,
) -> Result<(), D::Error> {
    if name.len() < 3 || name.len() > 255 {
        return Err(D::Error::custom(
            "Name must be between 3 and 255 characters",
        ));
    }

    if description.len() < 3 || description.len() > 2048 {
        return Err(D::Error::custom(
            "Description must be between 3 and 2048 characters",
        ));
    }

    if !NAME_RE.is_match(name) {
        return Err(D::Error::custom(
            "Name can only contain letters, numbers, and underscores",
        ));
    }

    if !DESCRIPTION_RE.is_match(description) {
        return Err(D::Error::custom(
            "Description can only contain letters and numbers",
        ));
    }

    Ok(())
}
