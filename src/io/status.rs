use actix_web::http::StatusCode as HttpCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    // Error
    BadInput,

    NoToken,
    BadToken,
    MalformedToken,
    ExpiredToken,

    DataNotFound,
    FailedToCreateData,
    InternalServerError,
    BadLoginCredentials,

    // Success
    CreatedOrganization,
    OrderedProduts,
    Ok,
    GoodLogin,
}

impl Status {
    pub fn http(self) -> HttpCode {
        match self {
            Self::BadInput => HttpCode::BAD_REQUEST,

            Self::NoToken => HttpCode::FORBIDDEN,
            Self::BadToken => HttpCode::FORBIDDEN,
            Self::MalformedToken => HttpCode::FORBIDDEN,
            Self::ExpiredToken => HttpCode::FORBIDDEN,

            Self::DataNotFound => HttpCode::UNAUTHORIZED,
            Self::FailedToCreateData => HttpCode::UNAUTHORIZED,
            Self::InternalServerError => HttpCode::INTERNAL_SERVER_ERROR,
            Self::BadLoginCredentials => HttpCode::UNAUTHORIZED,

            Self::CreatedOrganization => HttpCode::CREATED,
            Self::Ok => HttpCode::OK,
            Self::OrderedProduts => HttpCode::CREATED,
            Self::GoodLogin => HttpCode::OK,
        }
    }
}
