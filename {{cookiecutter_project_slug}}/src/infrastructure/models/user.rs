use crate::domain::models::user::{CreateUser, UpdateUser, User};
use crate::infrastructure::schema::users;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct UserDiesel {
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

impl From<User> for UserDiesel {
    fn from(t: User) -> Self {
        Self {
            id: t.id,
            email: t.email,
            phone_number: t.phone_number,
            password_hash: t.password_hash,
            first_name: t.first_name,
            last_name: t.last_name,
            username: t.username,
            role: t.role,
            is_active: t.is_active,
            is_verified: t.is_verified,
            profile_image: t.profile_image,
            date_of_birth: t.date_of_birth,
            gender: t.gender,
            last_login: t.last_login,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDiesel {
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

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUserDiesel {
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

impl From<UserDiesel> for User {
    fn from(d: UserDiesel) -> Self {
        Self {
            id: d.id,
            email: d.email,
            phone_number: d.phone_number,
            password_hash: d.password_hash,
            first_name: d.first_name,
            last_name: d.last_name,
            username: d.username,
            role: d.role,
            is_active: d.is_active,
            is_verified: d.is_verified,
            profile_image: d.profile_image,
            date_of_birth: d.date_of_birth,
            gender: d.gender,
            last_login: d.last_login,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }
    }
}

impl From<CreateUser> for CreateUserDiesel {
    fn from(t: CreateUser) -> Self {
        Self {
            id: Uuid::new_v4(),
            email: t.email,
            phone_number: t.phone_number,
            password_hash: t.password_hash,
            first_name: t.first_name,
            last_name: t.last_name,
            username: t.username,
            role: t.role,
            is_active: t.is_active,
            is_verified: t.is_verified,
            profile_image: t.profile_image,
            date_of_birth: t.date_of_birth,
            gender: t.gender,
        }
    }
}

impl From<&CreateUser> for CreateUserDiesel {
    fn from(t: &CreateUser) -> Self {
        Self {
            id: Uuid::new_v4(),
            email: t.email.clone(),
            phone_number: t.phone_number.clone(),
            password_hash: t.password_hash.clone(),
            first_name: t.first_name.clone(),
            last_name: t.last_name.clone(),
            username: t.username.clone(),
            role: t.role.clone(),
            is_active: t.is_active,
            is_verified: t.is_verified,
            profile_image: t.profile_image.clone(),
            date_of_birth: t.date_of_birth,
            gender: t.gender.clone(),
        }
    }
}

impl From<UpdateUser> for UpdateUserDiesel {
    fn from(t: UpdateUser) -> Self {
        Self {
            id: t.id,
            email: t.email,
            phone_number: t.phone_number,
            password_hash: t.password_hash,
            first_name: t.first_name,
            last_name: t.last_name,
            username: t.username,
            role: t.role,
            is_active: t.is_active,
            is_verified: t.is_verified,
            profile_image: t.profile_image,
            date_of_birth: t.date_of_birth,
            gender: t.gender,
        }
    }
}

impl From<&UpdateUser> for UpdateUserDiesel {
    fn from(t: &UpdateUser) -> Self {
        Self {
            id: t.id,
            email: t.email.clone(),
            phone_number: t.phone_number.clone(),
            password_hash: t.password_hash.clone(),
            first_name: t.first_name.clone(),
            last_name: t.last_name.clone(),
            username: t.username.clone(),
            role: t.role.clone(),
            is_active: t.is_active,
            is_verified: t.is_verified,
            profile_image: t.profile_image.clone(),
            date_of_birth: t.date_of_birth,
            gender: t.gender.clone(),
        }
    }
}

impl From<CreateUserDiesel> for User {
    fn from(d: CreateUserDiesel) -> Self {
        Self {
            id: Uuid::new_v4(),
            email: d.email,
            phone_number: d.phone_number,
            password_hash: d.password_hash,
            first_name: d.first_name,
            last_name: d.last_name,
            username: d.username,
            role: d.role,
            is_active: d.is_active,
            is_verified: d.is_verified,
            profile_image: d.profile_image,
            date_of_birth: d.date_of_birth,
            gender: d.gender,
            last_login: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }
}
