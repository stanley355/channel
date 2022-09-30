// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Uuid,
        owner_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        channel_name -> Varchar,
        slug -> Varchar,
        subscribers -> Int4,
        posts_number -> Int4,
        subscription_price -> Int4,
    }
}
