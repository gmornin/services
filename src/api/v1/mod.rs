pub mod account;
pub mod trigger;
pub mod usercontent;
mod error;
mod responses;
pub use error::*;
pub use responses::*;

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(account::scope())
        .service(trigger::scope())
        .service(usercontent::scope())
}
