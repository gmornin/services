use actix_web::Scope;

pub mod generic;

pub fn scope() -> Scope {
    Scope::new("gmt").service(generic::scope())
}
