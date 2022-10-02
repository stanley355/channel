use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::types::PostType;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreatePostPayload {
    pub channels_id: i32,
    pub img_url: String,
    pub description: String,
    pub post_type: PostType,
    pub is_free: bool,
}
