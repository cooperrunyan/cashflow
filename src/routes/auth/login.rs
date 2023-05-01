use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use serde_json::json;

use crate::{
    auth,
    io::{
        output::{error, success},
        Status,
    },
    prisma::{self, PrismaClient},
};

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
        Err(e) => return error::new(Status::InternalServerError, e).finish(),
        Ok(res) => match res {
            None => return error::new(Status::BadLoginCredentials, "Unavailable").finish(),
            Some(user) => user,
        },
    };

    match auth::check_hash(hashed, user.password) {
        false => error::new(Status::BadLoginCredentials, "Bad login").finish(),
        true => success::new(Status::GoodLogin, "ok")
            .data(json!( {
                "id": user.id,
                "email": user.email,
            }))
            .cookie(auth::jwt::gen_cookie(user.id, user.email))
            .finish(),
    }
}
