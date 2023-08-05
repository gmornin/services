use actix_web::Scope;

pub mod compile;
pub mod generic;
pub mod publish;

pub fn scope() -> Scope {
    Scope::new("tex")
        .service(generic::scope())
        .service(compile::scope())
        .service(publish::scope())
}
