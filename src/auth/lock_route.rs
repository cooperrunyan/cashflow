use actix_web::{cookie::Cookie, HttpRequest, HttpResponse};

use crate::io::{output::error, Status};

use super::jwt;

pub fn lock(req: HttpRequest) -> Result<jwt::Payload, HttpResponse> {
    let token = req.headers().get("Authorization");

    match token {
        None => Err(error::new(Status::NoToken, "No Authorization header found").finish()),

        Some(token) => {
            let cookie = &token.to_str().unwrap().replace("Bearer ", "");

            match jwt::decode(cookie) {
                Err(error) => {
                    return Err(error::new(Status::MalformedToken, error)
                        .cookie(Cookie::new("jwt", ""))
                        .finish())
                }

                Ok(payload) => {
                    if payload.is_exp() {
                        return Err(error::new(
                            Status::ExpiredToken,
                            "Token expired, log in again",
                        )
                        .cookie(Cookie::new("jwt", ""))
                        .finish());
                    }

                    if payload.user_id.is_empty() {
                        return Err(
                            error::new(Status::BadToken, "Token contains invalid fields")
                                .cookie(Cookie::new("jwt", ""))
                                .finish(),
                        );
                    }

                    Ok(payload)
                }
            }
        }
    }
}
