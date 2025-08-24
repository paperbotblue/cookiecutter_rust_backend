// @generated automatically by Diesel CLI.

diesel::table! {
    permissions (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 512]
        token -> Varchar,
        family_id -> Uuid,
        issued_at -> Timestamptz,
        expires_at -> Timestamptz,
        is_revoked -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    role_permissions (role_id, permission_id) {
        role_id -> Uuid,
        permission_id -> Uuid,
        description -> Text,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    todos (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Nullable<Varchar>,
        #[max_length = 100]
        username -> Nullable<Varchar>,
        #[max_length = 50]
        role -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        is_verified -> Nullable<Bool>,
        profile_image -> Nullable<Text>,
        date_of_birth -> Nullable<Date>,
        #[max_length = 20]
        gender -> Nullable<Varchar>,
        last_login -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    refresh_tokens,
    role_permissions,
    roles,
    todos,
    users,
);
