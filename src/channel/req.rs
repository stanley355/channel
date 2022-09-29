use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, AsExpression)]
pub struct CreateChannelReq {
    pub owner_id: String,
    pub channel_name: String,
}
