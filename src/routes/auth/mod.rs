use actix_web::web;

mod login;
use login::*;

mod signup;
use signup::*;

pub fn map() -> actix_web::Scope {
    web::scope("/auth").service(signup).service(login)
}
