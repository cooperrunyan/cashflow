use std::str::FromStr;

use actix_web::{
    cookie::Cookie,
    http::header::{HeaderMap, HeaderName, HeaderValue},
    HttpResponse, HttpResponseBuilder,
};
use serde::Serialize;
use serde_json::Value;

use super::Status;

#[derive(Debug, Clone)]
pub struct Success<'c> {
    payload: SuccessPayload,
    headers: HeaderMap,
    cookies: Vec<Cookie<'c>>,
}

#[derive(Serialize, Debug, Clone)]
struct SuccessPayload {
    status: Status,
    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl<'c> Success<'c> {
    pub fn new(status: Status, msg: impl ToString) -> Self {
        Self {
            payload: SuccessPayload {
                status,
                message: msg.to_string(),
                data: None,
            },
            headers: HeaderMap::new(),
            cookies: vec![],
        }
    }

    pub fn data(&mut self, data: Value) -> &mut Self {
        self.payload.data = Some(data);
        self
    }

    pub fn header(&mut self, name: impl ToString, value: impl ToString) -> &mut Self {
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

pub fn new<'c>(status: Status, msg: impl ToString) -> Success<'c> {
    Success::new(status, msg)
}
