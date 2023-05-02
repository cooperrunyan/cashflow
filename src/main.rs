#[macro_use(lazy_static)]
extern crate lazy_static;

#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};

pub use prisma;

pub use env::ENV;
pub use response::*;
pub use services::*;

pub mod auth;
pub mod env;
pub mod middleware;
pub mod parsers;
pub mod response;
pub mod routes;
mod services;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    ENV.port;

    let prisma_client = make_client().await;
    let prisma_context = web::Data::new(prisma_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::cors())
            .wrap(middleware::logger())
            .wrap(middleware::trailing_slash())
            .service(routes::auth::map())
            .service(routes::customer::map())
            .app_data(prisma_context.clone())
    });

    debug!("BINDING to {}", ENV.port);

    let server = match server.bind(("127.0.0.1", ENV.port)) {
        Ok(server) => {
            trace!("BINDED to {}", ENV.port);
            server
        }
        Err(e) => {
            error!("");
            error!("BIND FAILED on {}", ENV.port);
            error!("");
            panic!("{e}")
        }
    };

    let server = server.run();
    info!("RUNNING on {}", ENV.port);

    server.await
}

async fn make_client() -> prisma::PrismaClient {
    match prisma::PrismaClient::_builder().build().await {
        Ok(c) => c,
        Err(e) => {
            panic!("{e}")
        }
    }
}
