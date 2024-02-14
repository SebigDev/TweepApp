use crate::dtos::dto::TweetDto;
use crate::model::{like_model::Like, tweet_comment::Comment};
use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: Option<ObjectId>,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub likes: Vec<Like>,
    pub comments: Vec<Comment>,
}

impl Tweet {
    pub fn new(message: &str, user_id: &str) -> Tweet {
        Tweet {
            id: None,
            user_id: Some(ObjectId::parse_str(&user_id).unwrap()),
            created_at: Utc::now(),
            message: message.to_string(),
            likes: vec![],
            comments: vec![],
        }
    }
    /// Transforms <b>Tweet</b> to <b>TweetDto</b> using mapping.
    pub fn map(&self) -> TweetDto {
        TweetDto {
            id: self.id.unwrap().to_hex(),
            user_id: self.user_id.unwrap().to_hex(),
            created_at: self.created_at,
            message: self.message.clone(),
            likes: self.likes.clone().into_iter().map(|l| l.map()).collect(),
            comments: self.comments.clone().into_iter().map(|c| c.map()).collect(),
        }
    }

    /// Adds like to a tweet
    pub fn add_like(&mut self, like: Like) {
        self.likes.push(like);
    }

    ///Removes like from a tweet
    pub fn remove_like(&mut self, id: &str) {
        let _id = ObjectId::parse_str(id).expect("Invalid like id provided");
        self.likes
            .retain(|a| !a.id.unwrap().to_hex().eq(&_id.to_hex()));
    }

    /// Adds comments to tweet
    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment)
    }

    /// Removes comments from a tweet
    pub fn remove_comment(&mut self, comment_id: &str) {
        let _id = ObjectId::parse_str(comment_id).expect("Invalid comment id provided");
        self.comments
            .retain(|c| !c.id.unwrap().to_hex().eq(&_id.to_hex()));
    }
}

pub trait TweetActions {
    fn tweet(&self, user_id: String) -> Option<Tweet>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetActions for TweetRequest {
    fn tweet(&self, user_id: String) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message, &user_id)),
            None => None,
        }
    }
}
