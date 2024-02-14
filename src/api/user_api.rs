use actix_web::{
    post,
    web::{Data, Json, ReqData},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use jwt::RegisteredClaims;

use crate::auths::utils::get_user_id;
use crate::{
    auths::auth::{AuthData, ChangePasswordRequest, CreateUser},
    model::auth_model::User,
    repo::user_repo::UserRepo,
};

#[post("/api/v1/user/register")]
pub async fn register(db: Data<UserRepo<User>>, new_user: Json<CreateUser>) -> impl Responder {
    let data: CreateUser = new_user.into_inner();
    let user = User::new(&data.email, &data.password);
    let result = db.register(user).await;
    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[post("/api/v1/user/login")]
pub async fn login(db: Data<UserRepo<User>>, auth: Json<AuthData>) -> impl Responder {
    let user: AuthData = auth.into_inner();
    let result = db.valid_user(&user).await;
    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
    }
}

#[post("/user/change-password")]
pub async fn change_password(
    db: Data<UserRepo<User>>,
    req: Json<ChangePasswordRequest>,
    claims: Option<ReqData<RegisteredClaims>>,
    request: HttpRequest,
) -> impl Responder {
    match get_user_id(claims) {
        Ok(id) => id,
        Err(err) => return HttpResponse::Unauthorized().body(err.to_string()),
    };
    let password_request: ChangePasswordRequest = req.into_inner();
    let result = db.change_password(password_request).await;
    match result {
        Ok(resp) => {
            let mut mut_request = request.extensions_mut();
            mut_request.clear();
            HttpResponse::Ok().json(resp)
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[post("/user/logout")]
pub async fn signout(req: HttpRequest) -> impl Responder {
    let mut request = req.extensions_mut();
    request.clear();
    let message = Box::new("Logged out successfully");
    HttpResponse::Ok().json(message)
}
