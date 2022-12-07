use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreateChannelPayload {
    pub owner_id: String,
    pub profile_img_url: String,
    pub channel_name: String,
    pub subscription_price: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckChannelParam {
    pub id: Option<i32>,
    pub owner_id: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateChannelParam {
    pub channel_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateChannelPayload {
    pub channel_id: i32,
    pub channel_name: Option<String>,
    pub subscription_price: Option<i32>,
    pub profile_img_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchSimilarChannelQuery {
    pub channel_name: String,
}
