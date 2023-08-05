use actix_web::Scope;

mod createcoll;
mod publish;

pub fn scope() -> Scope {
    Scope::new("/v1").service(publish::publish)
}
