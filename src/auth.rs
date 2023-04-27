use crate::config::CONFIG;
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JwtPayload {
    pub user_id: String,
    pub email: String,
    exp: i64,
}

impl JwtPayload {
    pub fn new(user_id: String, email: String) -> Self {
        Self {
            user_id,
            email,
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

pub fn create_jwt(payload: JwtPayload) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());

    encode(&Header::default(), &payload, &encoding_key)
}

pub fn decode_jwt(token: &str) -> Result<JwtPayload, Error> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());

    decode::<JwtPayload>(token, &decoding_key, &Validation::default()).map(|data| data.claims)
}

// uses argon2i
pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.auth_salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    static EMAIL: &str = "test@test.com";

    #[test]
    fn it_hashes_a_password() {
        let password = "password";
        let hashed = hash(password);
        assert_ne!(password, hashed);
    }

    #[test]
    fn it_matches_2_hashed_passwords() {
        let password = "password";
        let hashed = hash(password);
        let hashed_again = hash(password);
        assert_eq!(hashed, hashed_again);
    }

    #[test]
    fn it_creates_a_jwt() {
        let private_claim = JwtPayload::new("hello".to_string(), EMAIL.into());
        let jwt = create_jwt(private_claim);
        assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_jwt() {
        let private_claim = JwtPayload::new("hello".to_string(), EMAIL.into());
        let jwt = create_jwt(private_claim.clone()).unwrap();
        let decoded = decode_jwt(&jwt).unwrap();
        assert_eq!(private_claim, decoded);
    }
}
