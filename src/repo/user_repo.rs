use crate::{
    auths::auth::{AuthData, ChangePasswordRequest},
    dtos::dto::UserDto,
    errors::error::TweetError,
    model::{auth_model::User, docs::update_user_document},
};
use bson::{doc, oid::ObjectId};
use mongodb::{Collection, Cursor};

pub struct UserRepo<User> {
    pub collection: Collection<User>,
}

impl UserRepo<User> {
    pub async fn register(&self, user: User) -> Result<UserDto, TweetError> {
        let user_result = self.get_user_by_email(&user.email).await;
        match user_result {
            Ok(mut resp) => {
                while resp.advance().await.unwrap() {
                    return Err(TweetError::BadRequest(format!(
                        "User with {} already exists",
                        user.email
                    )));
                }
            }
            Err(_) => return Err(TweetError::InternalServerError),
        };
        let _user = self
            .collection
            .insert_one(user, None)
            .await
            .map_err(|_| TweetError::InternalServerError)
            .ok()
            .unwrap();

        let id = match _user.inserted_id.as_object_id() {
            Some(id) => id.to_hex(),
            None => return Err(TweetError::BadRequest("Error registering user".into())),
        };
        return Ok(UserDto {
            id,
            message: "Your registration was successful".into(),
        });
    }

    pub async fn valid_user(&self, auth: &AuthData) -> Result<String, TweetError> {
        let user_result = self.get_user_by_email(&auth.email).await;
        match user_result {
            Ok(mut user) => {
                while user.advance().await.unwrap() {
                    let _user = match user.deserialize_current() {
                        Ok(user) => user,
                        Err(_) => {
                            return Err(TweetError::Unauthorized("User data is not valid".into()))
                        }
                    };
                    let token_option = _user.generate_token(&auth.password);
                    let token = match token_option {
                        Some(_token) => _token,
                        None => return Err(TweetError::Unauthorized("authentication failed, please check that email and/or password are correct".into()))
                        
                    };
                    return Ok(token);
                }
                return Err(TweetError::Unauthorized(
                    format!("No user with {} found.", &auth.email).into(),
                ));
            }
            Err(_) => return Err(TweetError::InternalServerError),
        };
    }

    pub async fn change_password(
        &self,
        request: ChangePasswordRequest,
    ) -> Result<String, TweetError> {
        let verify = self.get_user_by_email(&request.email).await;

        match verify {
            Err(_) => return Err(TweetError::BadRequest("User verification failed".into())),
            Ok(mut v_result) => {
                while v_result.advance().await.unwrap() {
                    let user_result = v_result.deserialize_current();
                    let mut user = match user_result {
                        Err(_) => return Err(TweetError::BadRequest("No user found".into())),
                        Ok(new_user) => new_user,
                    };

                    if user.verify_password(&request.password) {
                        if user.verify_password(&request.new_password) {
                            return Err(TweetError::BadRequest(
                                "Old and new password must not be the same".into(),
                            ));
                        }
                        user.update_password(&request.new_password);
                        let _id = ObjectId::parse_str(user.id.unwrap().to_hex().as_str())
                            .expect("Invalid User Id provided.");
                        let query = doc! {
                            "_id": _id
                        };
                        let _update_password = self
                            .collection
                            .update_one(query, update_user_document(&user), None)
                            .await
                            .map_err(|_| TweetError::InternalServerError);

                        let response = match _update_password {
                            Ok(_) => String::from("Password updated successfully."),
                            Err(err) => return Err(err),
                        };
                        return Ok(response);
                    }
                    return Err(TweetError::BadRequest("Invalid password provided.".into()));
                }
                return Err(TweetError::BadRequest("Error changing password.".into()));
            }
        };
    } 

    /// Get user by email address
    async fn get_user_by_email(&self, email: &str) -> Result<Cursor<User>, TweetError> {
        let filter = doc! {"email": &email};
        self.collection
            .find(filter, None)
            .await
            .map_err(|_| TweetError::InternalServerError)
    }
}
