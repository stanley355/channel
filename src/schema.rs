// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Int4,
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

diesel::table! {
    posts (id) {
        id -> Varchar,
        channels_id -> Int4,
        created_at -> Timestamp,
        img_url -> Varchar,
        description -> Varchar,
        likes -> Int4,
        post_type -> Varchar,
        is_free -> Bool,
    }
}

diesel::joinable!(posts -> channels (channels_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    posts,
);
