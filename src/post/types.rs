use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PostType {
    Image,
    Video,
}

impl fmt::Display for PostType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PostType::Image => write!(f, "Image"),
            PostType::Video => write!(f, "Video"),
        }
    }
}