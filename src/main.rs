#[macro_use(lazy_static)]
extern crate lazy_static;

use actix_web::{web, App, HttpServer};

use config::CONFIG;

mod auth;
mod config;
mod io;
mod middleware;
mod prisma;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let prisma_client = make_client().await;
    let prisma_context = web::Data::new(prisma_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::cors())
            .wrap(middleware::logger())
            .wrap(middleware::trailing_slash())
            .app_data(prisma_context.clone())
            .service(routes::auth::map())
            .service(routes::customer::map())
    });

    let server = match server.bind(("127.0.0.1", CONFIG.port)) {
        Ok(server) => server,
        Err(e) => panic!("{e}"),
    };

    let server = server.run();

    println!("Server running on port {}", CONFIG.port);

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
