use crate::model::{like_model::Like, tweet_comment::Comment, tweet_model::Tweet};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LikeDto {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub tweet_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetDto {
    pub id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<LikeDto>,
    pub comments: Vec<CommentDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentDto {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub tweet_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub message: String,
}

impl TweetDto {
    /// Transforms <b>TweetDto</b> to <b>Tweet</b> using mapping.
    pub fn to_tweet(self) -> Tweet {
        Tweet {
            id: Some(ObjectId::parse_str(&self.id).unwrap()),
            user_id: Some(ObjectId::parse_str(&self.id).unwrap()),
            message: self.message.to_owned(),
            created_at: self.created_at,
            likes: self.likes.into_iter().map(|l| l.to_like()).collect(),
            comments: self.comments.into_iter().map(|c| c.to_comment()).collect(),
        }
    }
}

impl LikeDto {
    /// Transforms <b>LikeDto</b> to <b>Like</b> using mapping.
    pub fn to_like(&self) -> Like {
        Like {
            id: Some(ObjectId::parse_str(&self.id).unwrap()),
            created_at: self.created_at,
            tweet_id: Some(ObjectId::parse_str(&self.tweet_id).unwrap()),
        }
    }
}

impl CommentDto {
     /// Transforms <b>CommentDto</b> to <b>Comment</b> using mapping.
    pub fn to_comment(&self) -> Comment {
        Comment {
            id: Some(ObjectId::parse_str(&self.id).unwrap()),
            created_at: self.created_at,
            message: self.message.clone(),
            tweet_id: Some(ObjectId::parse_str(&self.tweet_id).unwrap()),
        }
    }
}
