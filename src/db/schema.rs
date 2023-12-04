// @generated automatically by Diesel CLI.

diesel::table! {
    blog (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    user_login_history (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
        token -> Varchar,
        expire_at -> Timestamp,
    }
}
