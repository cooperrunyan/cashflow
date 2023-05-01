use actix_web::middleware::{NormalizePath, TrailingSlash};

pub fn trailing_slash() -> NormalizePath {
    NormalizePath::new(TrailingSlash::Trim)
}
