use actix_web::Scope;

pub mod services;

pub fn scope() -> Scope {
    Scope::new("/api").service(services::scope())
}
