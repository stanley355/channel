use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelTokenRes {
    pub token: String,
}

impl ChannelTokenRes {
    pub fn new(token: String) -> Self {
        ChannelTokenRes { token }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelErrorRes {
    pub error: String,
}
impl ChannelErrorRes {
    pub fn new(error: String) -> Self {
        ChannelErrorRes { error }
    }
}
