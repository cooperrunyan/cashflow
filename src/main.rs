#[macro_use(lazy_static)]
extern crate lazy_static;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};

mod prisma;
use prisma::*;

mod auth;
mod config;
mod errors;
mod formatters;
mod routes;

const PORT: u16 = 8000;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("PORT: {PORT}");

    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());

    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".domain.com"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(client.clone())
            .service(routes::analyze)
            .service(routes::setup_institution)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
