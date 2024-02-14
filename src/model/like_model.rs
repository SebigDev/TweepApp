use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::dtos::dto::LikeDto;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Like {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_at: DateTime<Utc>,
    pub tweet_id: Option<ObjectId>,
}

impl Like {
    /// Transforms <b>Like</b> to <b>LikeDto</b> using mapping.
    pub fn map(&self) -> LikeDto {
        LikeDto {
            id: self.id.unwrap().to_hex(),
            created_at: self.created_at,
            tweet_id: self.tweet_id.unwrap().to_hex(),
        }
    }
    pub fn new(tweet_id: &str) -> Self {
        Self {
            id: Some(ObjectId::new()),
            created_at: Utc::now(),
            tweet_id: Some(ObjectId::parse_str(tweet_id).unwrap()),
        }
    }
}
