pub mod account;
mod error;
mod responses;
pub mod storage;
pub mod trigger;
pub mod usercontent;
pub use error::*;
pub use responses::*;

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(account::scope())
        .service(trigger::scope())
        .service(storage::scope())
}
