use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreateChannelPayload {
    pub owner_id: String,
    pub channel_name: String,
    pub subscription_price: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckChannelParam {
    pub owner_id: Option<String>,
    pub slug: Option<String>,
}
