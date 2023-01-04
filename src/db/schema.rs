// @generated automatically by Diesel CLI.

diesel::table! {
    blogs (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
