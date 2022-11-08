use super::types::PostType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatePostPayload {
    pub channels_id: i32,
    pub channels_slug: String,
    pub img_url: String,
    pub description: String,
    pub post_type: PostType,
    pub is_free: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewChannelPostParam {
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewHomePostsPayload {
    pub subscriptions: Vec<i32>,
}
