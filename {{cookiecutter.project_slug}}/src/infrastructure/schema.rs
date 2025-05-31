// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Uuid,
        description -> Text,
        is_completed -> Bool,
        created_at -> Timestamptz,
    }
}
