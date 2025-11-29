use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePasswordDTO {
    pub otp: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserDTO {
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub role: String,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub profile_image: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserDTO {
    pub id: Uuid,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub role: String,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub profile_image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: Uuid,
    pub email: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub role: String,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub profile_image: Option<String>,
    pub otp: Option<String>,
    pub otp_expiry: Option<DateTime<Utc>>,
    pub otp_attempts: Option<i32>,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
