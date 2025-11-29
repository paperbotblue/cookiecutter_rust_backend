use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
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

    pub otp: Option<String>,
    pub otp_expiry: Option<DateTime<Utc>>,
    pub otp_attempts: Option<i32>,

    pub profile_image: Option<String>,

    pub last_login: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct CreateUser {
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

pub struct UpdateUser {
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

pub struct UpdatePassword {
    pub password: String,
    pub otp: String,
}
