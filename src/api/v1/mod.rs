pub mod account;
use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/v1").service(account::scope())
}
