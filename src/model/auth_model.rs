use std::time::{self, Duration, SystemTime};

use argonautica::{Hasher, Verifier};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use jwt::{claims::RegisteredClaims, header::HeaderType, Header, SignWithKey, Token};
use serde::{Deserialize, Serialize};

use crate::auths::utils::get_jwt_key;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_at: DateTime<Utc>,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(email: &str, password: &str) -> Self {
        User {
            id: None,
            created_at: Utc::now(),
            email: email.to_string(),
            password: Self::hash_password(password),
        }
    }

    pub fn update_password(&mut self, new_password: &str) {
        self.password = Self::hash_password(new_password);
    }
    /// Hashes the password with Hasher
    pub fn hash_password(password: &str) -> String {
        let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY not provided");
        let mut hasher = Hasher::default();
        hasher
            .with_password(password)
            .with_secret_key(secret)
            .hash()
            .unwrap()
      
    }

    /// Verifies the password using the verifier algorithm
    pub fn verify_password(&self, password: &str) -> bool {
        let secret = std::env::var("SECRET_KEY").expect("SECRET_KEY not provided");
        let mut verifier = Verifier::default();
        verifier
            .with_hash(&self.password)
            .with_password(password)
            .with_secret_key(secret)
            .verify()
            .unwrap()
    }

    /// Generates token string using the provided claims.
    pub fn generate_token(&self, password: &str) -> Option<String> {
        let key = get_jwt_key();
        let verify = self.verify_password(password);
        if verify {
            let headers = Header {
                type_: Some(HeaderType::JsonWebToken),
                algorithm: jwt::AlgorithmType::Hs256,
                ..Default::default()
            };

            let claims = RegisteredClaims {
                issuer: Some("TwitApp".to_string()),
                subject: Some("https://twitapp.com".to_string()),
                json_web_token_id: Some(self.id.unwrap().to_hex()),
                expiration: Some(
                    SystemTime::now()
                        .checked_add(Duration::from_secs(86400))
                        .unwrap()
                        .duration_since(time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                ),
                issued_at: Some(
                    SystemTime::now()
                        .duration_since(time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                ),
                ..Default::default()
            };

            let token = Token::new(headers, claims).sign_with_key(&key).unwrap();
            Some(token.as_str().into())
        } else {
            None
        }
    }
}
