use chrono::{DateTime, NaiveDate, Utc};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::validators::user::validate_user_fields;

#[derive(Serialize)]
pub struct CreateUserDTO {
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub profile_image: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateUserDTO {
    pub id: Uuid,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub profile_image: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserDTO {
    pub id: Uuid,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub profile_image: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawUserDTO {
    pub id: Option<Uuid>,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub profile_image: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl<'de> Deserialize<'de> for CreateUserDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawUserDTO::deserialize(deserializer)?;

        validate_user_fields::<D>()?;

        Ok(CreateUserDTO {
            email: raw.email,
            phone_number: raw.phone_number,
            password_hash: raw.password,
            first_name: raw.first_name,
            last_name: raw.last_name,
            username: raw.username,
            role: raw.role,
            is_active: raw.is_active,
            is_verified: raw.is_verified,
            profile_image: raw.profile_image,
            date_of_birth: raw.date_of_birth,
            gender: raw.gender,
        })
    }
}

impl<'de> Deserialize<'de> for UpdateUserDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawUserDTO::deserialize(deserializer)?;

        let id = raw
            .id
            .ok_or_else(|| D::Error::custom("ID is required for update"))?;

        validate_user_fields::<D>()?;

        Ok(UpdateUserDTO {
            id,
            email: raw.email,
            phone_number: raw.phone_number,
            password_hash: raw.password,
            first_name: raw.first_name,
            last_name: raw.last_name,
            username: raw.username,
            role: raw.role,
            is_active: raw.is_active,
            is_verified: raw.is_verified,
            profile_image: raw.profile_image,
            date_of_birth: raw.date_of_birth,
            gender: raw.gender,
        })
    }
}
