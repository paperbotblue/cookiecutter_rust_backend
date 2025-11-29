use crate::{
    api::dto::user::{CreateUserDTO, UpdatePasswordDTO, UpdateUserDTO, UserDTO},
    domain::models::user::{CreateUser, UpdatePassword, UpdateUser, User},
};

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            phone_number: value.phone_number,
            password_hash: value.password_hash,
            first_name: value.first_name,
            last_name: value.last_name,
            username: value.username,
            role: value.role,
            is_active: value.is_active,
            is_verified: value.is_verified,
            profile_image: value.profile_image,
            otp: value.otp,
            otp_attempts: value.otp_attempts,
            otp_expiry: value.otp_expiry,
            last_login: value.last_login,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<UpdatePasswordDTO> for UpdatePassword {
    fn from(value: UpdatePasswordDTO) -> Self {
        Self {
            password: value.password,
            otp: value.otp,
        }
    }
}

impl From<CreateUserDTO> for CreateUser {
    fn from(value: CreateUserDTO) -> Self {
        Self {
            email: value.email,
            phone_number: value.phone_number,
            password_hash: value.password_hash,
            first_name: value.first_name,
            last_name: value.last_name,
            username: value.username,
            role: value.role,
            is_active: value.is_active,
            is_verified: value.is_verified,
            profile_image: value.profile_image,
        }
    }
}

impl From<UpdateUserDTO> for UpdateUser {
    fn from(value: UpdateUserDTO) -> Self {
        Self {
            id: value.id,
            email: value.email,
            phone_number: value.phone_number,
            password_hash: value.password_hash,
            first_name: value.first_name,
            last_name: value.last_name,
            username: value.username,
            role: value.role,
            is_active: value.is_active,
            is_verified: value.is_verified,
            profile_image: value.profile_image,
        }
    }
}
