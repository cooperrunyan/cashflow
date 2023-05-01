use actix_web::HttpResponse;

use crate::io::{output::error, Status};

pub fn check(input: String) -> Result<String, HttpResponse> {
    let mut validate_phone_number = input.clone();

    if validate_phone_number.len() == 0 {
        return Err(error::new(Status::BadInput, "Phone number must not be empty").finish());
    }

    if validate_phone_number.remove(0) != '+' {
        return Err(error::new(Status::BadInput, "Phone number must begin with '+'").finish());
    }

    for ch in validate_phone_number.bytes() {
        if ch < 48 || ch > 57 {
            return Err(
                error::new(Status::BadInput, "Phone number must only contain numbers")
                    .input(input)
                    .finish(),
            );
        }
    }

    return Ok(input);
}
