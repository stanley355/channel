use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreateChannelPayload {
    pub owner_id: String,
    pub channel_name: String,
    pub subscription_price: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OwnerIdParam {
    pub owner_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelSlugParam {
    pub slug: String,
}
