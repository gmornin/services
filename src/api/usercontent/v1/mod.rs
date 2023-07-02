use actix_web::Scope;

mod diritems;
mod file;
// mod main;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(diritems::by_id)
        .service(file::by_id)
}
