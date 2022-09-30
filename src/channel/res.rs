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
