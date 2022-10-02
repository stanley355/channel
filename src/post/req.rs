use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreatePostPayload {
    pub channels_id: i32,
    pub img_url: String,
    pub description: String,
    pub post_type: String,
    pub is_free: bool,
}
