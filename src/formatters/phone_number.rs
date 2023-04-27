use actix_web::{http::StatusCode, HttpResponse};

use crate::errors::ServerError;

pub struct PhoneNumber {}

impl PhoneNumber {
    pub fn check(input: String) -> Result<String, HttpResponse> {
        let mut validate_phone_number = input.clone();

        if validate_phone_number.len() == 0 {
            return Err(ServerError::new(
                StatusCode::BAD_REQUEST,
                "Phone number must not be empty",
                Some("bad_input"),
                None::<String>,
            ));
        }

        if validate_phone_number.remove(0) != '+' {
            return Err(ServerError::new(
                StatusCode::BAD_REQUEST,
                format!("Phone number must begin with '+'",),
                None::<String>,
                Some(input),
            ));
        }

        for ch in validate_phone_number.bytes() {
            if ch < 48 || ch > 57 {
                return Err(ServerError::new(
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Phone number must only contain numbers -- input: '{}'",
                        input
                    ),
                    Some("bad_input"),
                    Some(input),
                ));
            }
        }

        return Ok(input);
    }
}
