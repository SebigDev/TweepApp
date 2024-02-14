use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, ReqData},
    HttpResponse, Responder,
};
use jwt::RegisteredClaims;

use crate::{
    auths::utils::get_user_id,
    model::{
        tweet_comment::{CommentAction, CommentRequest},
        tweet_model::{Tweet, TweetActions, TweetRequest},
    },
    repo::tweet_repo::TweetRepo,
};

#[post("/tweets")]
pub async fn create_tweet(
    request: Json<TweetRequest>,
    claims: Option<ReqData<RegisteredClaims>>,
    db: Data<TweetRepo<Tweet>>,
) -> impl Responder {
    let user_id = match get_user_id(claims) {
        Ok(id) => id,
        Err(err) => return HttpResponse::Unauthorized().body(err.to_string()),
    };
    let tweet = request.0.tweet(user_id).unwrap();
    let result = db.create_tweet(tweet).await;

    match result {
        Ok(resp) => HttpResponse::Created().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/tweets")]
pub async fn list_tweets(
    db: Data<TweetRepo<Tweet>>,
    claims: Option<ReqData<RegisteredClaims>>,
) -> impl Responder {
    let user_id = match get_user_id(claims) {
        Ok(id) => id,
        Err(err) => return HttpResponse::Unauthorized().body(err.to_string()),
    };
    let result = db.all_tweets(&user_id).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/tweets/{path}")]
pub async fn get_tweet(db: Data<TweetRepo<Tweet>>, path: Path<(String,)>) -> impl Responder {
    let id = path.0.as_str();
    if id.is_empty() {
        return HttpResponse::BadRequest().body(format!("Id not provided"));
    }
    let result = db.get_tweet(id).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/tweets/{path}")]
pub async fn delete_tweet(db: Data<TweetRepo<Tweet>>, path: Path<(String,)>) -> impl Responder {
    let id = path.0.as_str();
    if id.is_empty() {
        return HttpResponse::BadRequest().body(format!("Id not provided").to_string());
    }
    let result = db.delete_tweet(id).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/tweets/{path}/comment")]
pub async fn add_comment(
    db: Data<TweetRepo<Tweet>>,
    path: Path<(String,)>,
    request: Json<CommentRequest>,
) -> impl Responder {
    let tweet_id = path.0.as_str();
    let comment = request.into_inner().comment(tweet_id).unwrap();
    let result = db.add_comment(tweet_id, &comment.message).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/tweets/{tweet_id}/comment/{comment_id}")]
pub async fn delete_comment(
    db: Data<TweetRepo<Tweet>>,
    path: Path<(String, String)>,
) -> impl Responder {
    let tweet_id = path.0.as_str();
    let comment_id = path.1.as_str();
    if tweet_id.is_empty() || comment_id.is_empty() {
        return HttpResponse::BadRequest().body(format!("Id not provided").to_string());
    }
    let result = db.remove_comment(tweet_id, comment_id).await;

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
