// errors.rs
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
///Tweet errors 
pub enum TweetError {
    ///Internal server error due to network related issues
    #[display(fmt = "Internal Server Error, Please try later")]
    InternalServerError,

    ///Bad Request due to invalid request
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    ///Authentication error when authentication fails or unauthorised
    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for TweetError {
    fn error_response(&self) -> HttpResponse {
        match self {
            TweetError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            TweetError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            TweetError::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
        }
    }
}
