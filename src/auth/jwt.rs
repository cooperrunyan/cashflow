use actix_web::cookie::Cookie;
use chrono::{Duration, Utc};
use jsonwebtoken::{errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use serde::{Deserialize, Serialize};

use crate::ENV;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Payload {
    pub user_id: String,
    pub email: String,
    exp: i64,
}

impl Payload {
    fn new(user_id: String, email: String) -> Self {
        Self {
            user_id,
            email,
            exp: (Utc::now() + Duration::hours(ENV.jwt_exp)).timestamp(),
        }
    }

    pub fn is_exp(&self) -> bool {
        self.exp <= Utc::now().timestamp()
    }
}

pub fn encode(user_id: impl ToString, email: impl ToString) -> Result<String, Error> {
    let payload = Payload::new(user_id.to_string(), email.to_string());
    let encoding_key = EncodingKey::from_secret(&ENV.jwt_key.as_ref());
    let header = Header::new(Algorithm::HS512);

    debug!("Generating new JWT:");
    debug!("  {:#?}", payload);

    jsonwebtoken::encode(&header, &payload, &encoding_key)
}

pub fn decode(token: impl ToString) -> Result<Payload, Error> {
    let decoding_key = DecodingKey::from_secret(&ENV.jwt_key.as_ref());

    jsonwebtoken::decode::<Payload>(
        token.to_string().as_str(),
        &decoding_key,
        &Validation::new(Algorithm::HS512),
    )
    .map(|data| data.claims)
}

pub fn gen_cookie<'c>(id: impl ToString, email: impl ToString) -> Cookie<'c> {
    Cookie::build("jwt", encode(id.to_string(), email.to_string()).unwrap())
        // .domain("http://localhost:8000")
        .path("/")
        // .secure(true)
        // .http_only(true)
        .finish()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    static EMAIL: &str = "test@test.com";
    static USER_ID: &str = "1234";

    #[actix_rt::test]
    async fn it_creates_a_jwt() {
        let jwt = encode(USER_ID, EMAIL);
        assert!(jwt.is_ok());
    }

    #[actix_rt::test]
    async fn it_decodes_a_jwt() {
        let jwt = encode(USER_ID, EMAIL).unwrap();
        let decoded = decode(&jwt).unwrap();
        assert_eq!(USER_ID, decoded.user_id);
        assert_eq!(EMAIL, decoded.email);
    }
}
