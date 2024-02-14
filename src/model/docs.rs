use crate::model::tweet_model::Tweet;
use mongodb::bson::{self, doc, Document};

use super::auth_model::User;

/// Updates `Tweet` document in `Database`
pub fn update_tweet_document(tweet: &Tweet) -> Document {
    doc! {
         "$set":{
            "_id": bson::Bson::ObjectId(tweet.id.unwrap()),
            "message": bson::Bson::String(tweet.message.clone()),
            "created_at": bson::to_bson(&tweet.created_at).unwrap(),
            "likes": bson::to_bson(&tweet.likes).unwrap(),
            "comments": bson::to_bson(&tweet.comments).unwrap()
         }
    }
}

/// Updates `User` document in `Database`
pub fn update_user_document(user: &User) -> Document {
    doc! {
        "$set": {
            "_id" : bson::Bson::ObjectId(user.id.unwrap()),
            "password": bson::Bson::String(user.password.clone())
        }
    }
}
