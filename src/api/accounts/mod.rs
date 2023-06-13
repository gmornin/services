use actix_web::Scope;

mod v1;

pub fn scope() -> Scope {
    Scope::new("/accounts").service(v1::scope())
}
