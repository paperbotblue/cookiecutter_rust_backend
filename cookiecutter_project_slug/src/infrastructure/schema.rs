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
        client_id -> Uuid,
        client_type -> Text,
        is_revoked -> Bool,
        token -> Text,
        expires_at -> Timestamptz,
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

diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    refresh_tokens,
    role_permissions,
    roles,
    todos,
);
