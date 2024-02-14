extern crate actix_web;
extern crate log;

use actix_web::{middleware, web::Data, App, HttpServer};
use dbconn::MongoPool;
use model::{auth_model::User, tweet_model::Tweet};
use repo::{tweet_repo::TweetRepo, user_repo::UserRepo};
use routes::router;
use std::{env, io};

mod api;
mod auths;
mod dbconn;
mod dtos;
mod errors;
mod model;
mod repo;
mod routes;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let db = MongoPool::<Tweet>::connect().await;
    let user_db = MongoPool::<User>::connect().await;
    let pool = Data::new(TweetRepo {
        collection: db.collection,
    });
    let user_pool = Data::new(UserRepo {
        collection: user_db.collection,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(user_pool.clone())
            .app_data(pool.clone())
            .configure(router::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
