use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerError {
    message: String,
    error: Option<String>,
    input: Option<String>,
}

impl ServerError {
    pub fn new(
        status: StatusCode,
        msg: impl ToString,
        err: Option<impl ToString>,
        input: Option<impl ToString>,
    ) -> HttpResponse {
        HttpResponseBuilder::new(status).json(Self {
            message: msg.to_string(),
            error: err.map(|i| i.to_string()),
            input: input.map(|i| i.to_string()),
        })
    }
}
