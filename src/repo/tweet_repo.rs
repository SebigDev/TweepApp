use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::DeleteResult,
    Collection,
};

use crate::model::{
    docs::update_tweet_document, like_model::Like, tweet_comment::Comment, tweet_model::Tweet,
};
use crate::{dtos::dto::TweetDto, errors::error::TweetError};

pub struct TweetRepo<Tweet> {
    pub collection: Collection<Tweet>,
}

impl TweetRepo<Tweet> {
    pub async fn create_tweet(&self, tweet: Tweet) -> Result<TweetDto, TweetError> {
        let _tweet = self
            .collection
            .insert_one(tweet, None)
            .await
            .map_err(|_| TweetError::InternalServerError)
            .ok()
            .unwrap();

        let id = match _tweet.inserted_id.as_object_id() {
            Some(id) => id.to_hex(),
            None => return Err(TweetError::BadRequest("Error reading inserted id".into())),
        };

        let dto = self.get_tweet(&id).await.unwrap();
        return Ok(dto);
    }

    pub async fn all_tweets(&self, _user_id: &str) -> Result<Vec<TweetDto>, Error> {
        let user_id = ObjectId::parse_str(_user_id).expect("Invalid user_id");
        let filter = doc! {"user_id": user_id};
        let mut _tweets = self
            .collection
            .find(filter, None)
            .await
            .ok()
            .expect("Failed to retrieve all likes");
        let mut tweets = Vec::<Tweet>::new();
        while _tweets.advance().await.unwrap() {
            tweets.push(_tweets.deserialize_current().unwrap());
        }
        let dto = tweets
            .into_iter()
            .map(|t| t.map())
            .collect::<Vec<TweetDto>>();
        Ok(dto)
    }

    pub async fn get_tweet(&self, id: &str) -> Result<TweetDto, Error> {
        let _id = ObjectId::parse_str(id).expect("Invalid tweet Id provided");
        let filter = doc! {"_id": _id};
        let _tweet = self
            .collection
            .find(filter, None)
            .await
            .ok()
            .expect("Failed to retrieve like");
        let tweet_dto: TweetDto = _tweet.deserialize_current().unwrap().map();
        Ok(tweet_dto)
    }

    pub async fn delete_tweet(&self, id: &str) -> Result<DeleteResult, Error> {
        let _id = ObjectId::parse_str(id).expect("Invalid like Id provided");
        let filter = doc! {"_id": _id};
        let _tweet = self
            .collection
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Failed to retrieve tweet");
        Ok(_tweet)
    }

    pub async fn create_like(&self, tweet_id: &str) -> Result<TweetDto, Error> {
        let _id = ObjectId::parse_str(tweet_id).expect("Invalid tweet Id provided");
        let tweet_dto = self.get_tweet(tweet_id).await.unwrap();
        let mut tweet = tweet_dto.to_tweet();
        tweet.add_like(Like::new(tweet_id));
        let query = doc! {"_id": _id };
        let _like = self
            .collection
            .update_one(query, update_tweet_document(&tweet), None)
            .await
            .ok()
            .expect("Error creating like");
        Ok(tweet.map())
    }

    pub async fn remove_like(&self, tweet_id: &str, id: &str) -> Result<TweetDto, Error> {
        let _id = ObjectId::parse_str(tweet_id).expect("Invalid tweet Id provided");
        let tweet_dto = self.get_tweet(tweet_id).await.unwrap();
        let mut tweet = tweet_dto.to_tweet();
        tweet.remove_like(id);

        let query = doc! {"_id": _id };
        let _like = self
            .collection
            .update_one(query, update_tweet_document(&tweet), None)
            .await
            .ok()
            .expect("Error removing like");
        Ok(tweet.map())
    }

    pub async fn add_comment(&self, tweet_id: &str, message: &str) -> Result<TweetDto, Error> {
        let _id = ObjectId::parse_str(tweet_id).expect("Invalid tweet Id provided");
        let tweet_dto = self.get_tweet(tweet_id).await.unwrap();
        let mut tweet = tweet_dto.to_tweet();
        tweet.add_comment(Comment::new(tweet_id, message));

        let query = doc! {"_id": _id };
        let _like = self
            .collection
            .update_one(query, update_tweet_document(&tweet), None)
            .await
            .ok()
            .expect("Error adding comment");
        Ok(tweet.map())
    }

    pub async fn remove_comment(
        &self,
        tweet_id: &str,
        comment_id: &str,
    ) -> Result<TweetDto, Error> {
        let _id = ObjectId::parse_str(tweet_id).expect("Invalid tweet Id provided");
        let tweet_dto = self.get_tweet(tweet_id).await.unwrap();
        let mut tweet = tweet_dto.to_tweet();
        tweet.remove_comment(comment_id);

        let query = doc! {"_id": _id };
        let _like = self
            .collection
            .update_one(query, update_tweet_document(&tweet), None)
            .await
            .ok()
            .expect("Error removing comment");
        Ok(tweet.map())
    }
}
