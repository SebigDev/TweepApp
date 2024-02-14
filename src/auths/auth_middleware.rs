use super::utils::get_jwt_key;
use crate::errors::error::TweetError;
use actix_web::HttpMessage;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use chrono::Utc;
use jwt::{RegisteredClaims, VerifyWithKey};

/// Authentication validator using BearerAuth and ServiceRequest.
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token_string = credentials.token();

    let claims: Result<RegisteredClaims, TweetError> = token_string
        .verify_with_key(&get_jwt_key())
        .map_err(|_| TweetError::Unauthorized("Invalid token".into()));
    match claims {
        Ok(claim) => {
            let expired = is_token_expired(claim.expiration.unwrap_or_default());
            if expired {
                let config = req
                    .app_data::<bearer::Config>()
                    .cloned()
                    .unwrap_or_default()
                    .scope("/api/v1");
                return Err((
                    AuthenticationError::from(config)
                        .with_error_description("Token has expired")
                        .into(),
                    req,
                ));
            }
            req.extensions_mut().insert(claim);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("/api/v1");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

fn is_token_expired(ex: u64) -> bool {
    Some(ex).unwrap_or(0) < Utc::now().timestamp() as u64
}
