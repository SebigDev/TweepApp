use actix_web::web::ReqData;
use hmac::{Hmac, Mac};
use jwt::RegisteredClaims;
use sha2::Sha256;

use crate::errors::error::TweetError;

type HmacSha256 = Hmac<Sha256>;

/// Gets JWT Key using the HmacSha256
pub fn get_jwt_key() -> Hmac<Sha256> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not provided");
    let key: Hmac<Sha256> = HmacSha256::new_from_slice(jwt_secret.as_bytes()).unwrap();
    key
}

/// Gets the authenticated user identity from RegisteredClaims
pub fn get_user_id(claims: Option<ReqData<RegisteredClaims>>) -> Result<String, TweetError> {
    let token = match claims {
        Some(claim) => claim.into_inner(),
        None => {
            return Err(TweetError::Unauthorized(
                "authentication error occurred".into(),
            ))
        }
    };
    let user_id = match token.json_web_token_id {
        Some(claim) => claim,
        None => {
            return Err(TweetError::Unauthorized(
                "authentication error occurred".into(),
            ))
        }
    };
    Ok(user_id)
}
