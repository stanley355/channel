// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Uuid,
        owner_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        subscribers -> Int4,
        posts_number -> Int4,
    }
}
