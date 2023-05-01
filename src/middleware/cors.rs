use actix_cors::Cors;
use actix_web::http::header::*;

pub fn cors() -> Cors {
    Cors::default()
        .send_wildcard()
        // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".domain.com"))
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![AUTHORIZATION, ACCEPT])
        .allowed_header(CONTENT_TYPE)
        .max_age(3600)
}
