use actix_web::{
    post,
    delete,
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::{model::tweet_model::Tweet, repo::tweet_repo::TweetRepo};

#[post("/likes/{tweet_id}")]
pub async fn plus_one(db: Data<TweetRepo<Tweet>>, tweet_id: Path<(String,)>) -> impl Responder {
    let id = tweet_id.0.as_str();
    if id.is_empty() {
        return HttpResponse::BadRequest().body(format!("Id not provided"));
    }
    let result = db.create_like(id).await;

    match result {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/likes/{tweet_id}/{like_id}")]
pub async fn minus_one(db: Data<TweetRepo<Tweet>>, path: Path<(String, String,)>) -> impl Responder {
    let tweet_id = path.0.as_str();
    let like_id = path.1.as_str();
    if tweet_id.is_empty() {
        return HttpResponse::BadRequest().body(format!("tweet id not provided"));
    }
    if like_id.is_empty() {
        return HttpResponse::BadRequest().body(format!("like id not provided"));
    }
    let result = db.remove_like(tweet_id, like_id).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}