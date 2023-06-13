use actix_web::Scope;

pub mod accounts;
pub mod gmt;
pub mod storage;
pub mod triggers;
pub mod usercontent;

pub fn scope() -> Scope {
    Scope::new("/api")
        .service(accounts::scope())
        .service(storage::scope())
        .service(triggers::scope())
        .service(usercontent::scope())
        .service(gmt::scope())
}
