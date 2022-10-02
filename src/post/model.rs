


#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Post {
    id: i32,
    channels_id: i32,
    created_at: chrono::NaiveDateTime,
    img_url: String,
    description: String,
    likes: i32,
    post_type: String,
    is_free: bool,
}
