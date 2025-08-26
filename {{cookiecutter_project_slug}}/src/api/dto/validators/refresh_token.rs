// use std::sync::LazyLock;

// use regex::Regex;
// use serde::de::Error as DeError;
use serde::Deserializer;

// static NAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").expect(""));

pub fn validate_refresh_token_fields<'de, D: Deserializer<'de>>(// name: &str,
) -> Result<(), D::Error> {
    // if description.len() < 3 || description.len() > 2048 {
    //     return Err(D::Error::custom(
    //         "Description must be between 3 and 2048 characters",
    //     ));
    // }
    //
    // if !NAME_RE.is_match(name) {
    //     return Err(D::Error::custom(
    //         "Name can only contain letters, numbers, and underscores",
    //     ));
    // }

    Ok(())
}
