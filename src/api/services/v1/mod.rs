pub mod account;
pub mod storage;
pub mod trigger;
pub mod usercontent;

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(account::scope())
        .service(trigger::scope())
        .service(storage::scope())
        .service(usercontent::scope())
}
