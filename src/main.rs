#[macro_use(lazy_static)]
extern crate lazy_static;

#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};
use prisma;

pub use env::ENV;

mod auth;
mod env;
mod io;
mod middleware;
mod routes;

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

    debug!("Binding server to :{}", ENV.port);

    let server = match server.bind(("127.0.0.1", ENV.port)) {
        Ok(server) => {
            debug!("Server binded to :{}", ENV.port);
            server
        }
        Err(e) => {
            error!("");
            error!("Failed to bind to :{}", ENV.port);
            error!("");
            panic!("{e}")
        }
    };

    info!("Server running on port {}", ENV.port);
    let server = server.run();
    debug!("Server started successfully");

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
