pub use analyze::*;

mod analyze;

use actix_web::web;

pub fn map() -> actix_web::Scope {
    web::scope("/customer").service(analyze)
}
