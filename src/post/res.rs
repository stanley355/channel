use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct HomePost {
    id: i32,
    channels_id: i32,
    channels_slug: String,
    created_at: chrono::NaiveDateTime,
    img_url: String,
    description: String,
    likes: i32,
    post_type: String,
    is_free: bool,
    title: Option<String>,
    channel_name: String,
    profile_img_url: Option<String>,
}