use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::dtos::dto::CommentDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Option<ObjectId>,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub tweet_id: Option<ObjectId>,
}

impl Comment {
    pub fn new(tweet_id: &str, message: &str) -> Self {
        Self {
            id: Some(ObjectId::new()),
            message: message.to_string(),
            created_at: Utc::now(),
            tweet_id: Some(ObjectId::parse_str(&tweet_id).unwrap()),
        }
    }
    /// Transforms <b>Comment</b> to <b>CommentDo</b> using mapping.
    pub fn map(&self) -> CommentDto {
        CommentDto {
            id: self.id.unwrap().to_hex(),
            created_at: self.created_at,
            message: self.message.clone(),
            tweet_id: self.tweet_id.unwrap().to_hex(),
        }
    }
}

pub trait CommentAction {
    fn comment(&self, tweet_id: &str) -> Option<Comment>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentRequest {
    pub message: Option<String>,
}

impl CommentAction for CommentRequest {
    fn comment(&self, tweet_id: &str) -> Option<Comment> {
        match &self.message {
            Some(message) => Some(Comment::new(tweet_id, message.as_str())),
            None => None,
        }
    }
}
