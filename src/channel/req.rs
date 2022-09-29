use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CreateChannelReq {
    pub owner_id: String,
    pub channel_name: String,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct CheckChannelParam {
    pub owner_id: String,
    pub channel_name: String,
}
