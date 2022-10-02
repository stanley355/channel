use serde::{Deserialize, Serialize};
use super::types::PostType;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatePostPayload {
    pub channels_id: i32,
    pub img_url: String,
    pub description: String,
    pub post_type: PostType,
    pub is_free: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewChannelPostParam {
    pub channel_id: i32
}