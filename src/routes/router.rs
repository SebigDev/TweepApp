use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{
    api::{
        like_api::{minus_one, plus_one},
        tweet_api::{
            add_comment, create_tweet, delete_comment, delete_tweet, get_tweet, list_tweets,
        },
        user_api::{change_password, login, register, signout},
    },
    auths::auth_middleware::validator,
};

pub fn init(config: &mut web::ServiceConfig) {
    let auth_middleware = HttpAuthentication::bearer(validator);
    config.service(login);
    config.service(register);
    config.service(
        web::scope("/api/v1")
            .wrap(auth_middleware)
            .service(create_tweet)
            .service(list_tweets)
            .service(get_tweet)
            .service(delete_tweet)
            .service(plus_one)
            .service(minus_one)
            .service(add_comment)
            .service(delete_comment)
            .service(change_password)
            .service(signout),
    );
}