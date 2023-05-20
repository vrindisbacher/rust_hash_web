// @generated automatically by Diesel CLI.

diesel::table! {
    auth (id) {
        id -> Text,
        salt -> Text,
        hashed_value -> Text,
        encryption_algorithm -> Text,
    }
}
