use std::sync::LazyLock;

use regex::Regex;
use serde::de::Error as DeError;
use serde::Deserializer;

// Name: ASCII letters, digits, underscore, 1+ chars
static NAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9_ ]+$").expect("Invalid NAME_RE pattern"));

// Description: ASCII letters and digits only, 1+ chars
static DESCRIPTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9 ]+$").expect("Invalid DESCRIPTION_RE pattern"));

pub fn validate_role_fields<'de, D: Deserializer<'de>>(
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
