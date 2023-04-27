use actix_web::{web, App, HttpServer};
mod prisma;
use prisma::*;

mod routes;

const PORT: u16 = 8000;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("PORT: {PORT}");

    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(routes::analyze)
            .service(routes::setup_institution)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
