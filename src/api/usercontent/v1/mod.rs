use actix_web::Scope;

mod diritems;
mod exists;
mod file;
mod tree;
// mod main;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(diritems::by_id)
        .service(file::by_id)
        .service(tree::tree)
        .service(exists::by_id)
}
