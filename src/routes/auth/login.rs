use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use prisma::PrismaClient;
use serde_json::json;

use crate::auth;
use crate::response::*;

#[derive(serde::Serialize, serde::Deserialize)]
struct RequestBody {
    email: String,
    password: String,
}

#[post("/login")]
async fn login(client: Data<PrismaClient>, body: Json<RequestBody>) -> impl Responder {
    let hashed = auth::hash(body.password.clone());

    let user = match client
        .member()
        .find_unique(prisma::member::email::equals(body.email.clone()))
        .exec()
        .await
    {
        Err(e) => return error(Status::InternalServerError, e).finish(),
        Ok(res) => match res {
            None => return error(Status::BadLoginCredentials, "Unavailable").finish(),
            Some(user) => user,
        },
    };

    match auth::check_hash(hashed, user.password) {
        false => error(Status::BadLoginCredentials, "Bad login").finish(),
        true => success(Status::GoodLogin, "ok")
            .data(json!( {
                "id": user.id,
                "email": user.email,
            }))
            .cookie(auth::jwt::gen_cookie(user.id, user.email))
            .finish(),
    }
}
