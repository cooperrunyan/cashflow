use std::str::FromStr;

use actix_web::{
    cookie::Cookie,
    http::header::{HeaderMap, HeaderName, HeaderValue},
    HttpResponse, HttpResponseBuilder,
};
use serde::{Deserialize, Serialize};

use super::Status;

#[derive(Debug, Clone)]
pub struct ServerError<'c> {
    payload: ServerErrorPayload,
    headers: HeaderMap,
    cookies: Vec<Cookie<'c>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ServerErrorPayload {
    status: Status,
    error: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<String>,
}

impl<'c> ServerError<'c> {
    pub fn new(status: Status, msg: impl ToString) -> Self {
        Self {
            payload: ServerErrorPayload {
                status,
                error: msg.to_string(),
                input: None,
            },
            headers: HeaderMap::new(),
            cookies: vec![],
        }
    }

    pub fn input(&mut self, input: impl ToString) -> &mut Self {
        self.payload.input = Some(input.to_string());
        self
    }

    #[allow(unused)]
    pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
        self.headers.insert(
            HeaderName::from_str(name.to_string().as_str()).unwrap(),
            HeaderValue::from_str(value.to_string().as_str()).unwrap(),
        );
        self
    }

    pub fn cookie(&mut self, cookie: Cookie<'c>) -> &mut Self {
        self.cookies.append(&mut vec![cookie]);
        self
    }

    pub fn finish(&self) -> HttpResponse {
        let mut res = HttpResponseBuilder::new(self.payload.status.clone().http());

        for header in &self.headers {
            res.append_header(header);
        }

        for cookie in self.cookies.iter() {
            res.cookie(cookie.clone().into_owned());
        }

        res.json(&self.payload)
    }
}

pub fn new<'c>(status: Status, msg: impl ToString) -> ServerError<'c> {
    ServerError::new(status, msg)
}
