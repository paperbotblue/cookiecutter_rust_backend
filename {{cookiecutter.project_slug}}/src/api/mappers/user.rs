use crate::{
    api::dto::user::{CreateUserDTO, UserDTO, UpdateUserDTO},
    domain::{
        models::user::{CreateUser, User, UpdateUser},
        repositories::repository::ResultPaging,
    },
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
            date_of_birth: value.date_of_birth,
            gender: value.gender,
            last_login: value.last_login,
            created_at: value.created_at,
            updated_at: value.updated_at,
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
            date_of_birth: value.date_of_birth,
            gender: value.gender,
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
            date_of_birth: value.date_of_birth,
            gender: value.gender,
        }
    }
}

impl From<CreateUser> for CreateUserDTO {
    fn from(value: CreateUser) -> Self {
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
            date_of_birth: value.date_of_birth,
            gender: value.gender,
        }
    }
}

impl From<UpdateUser> for UpdateUserDTO {
    fn from(value: UpdateUser) -> Self {
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
            date_of_birth: value.date_of_birth,
            gender: value.gender,
        }
    }
}

impl From<ResultPaging<User>> for ResultPaging<UserDTO> {
    fn from(value: ResultPaging<User>) -> Self {
        Self {
            total: value.total,
            items: value.items.into_iter().map(Into::into).collect(),
        }
    }
}
