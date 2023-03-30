use actix_web::Scope;

mod mkdir;
mod overwrite;
mod read;
mod write_new;

pub fn scope() -> Scope {
    Scope::new("/storage/{token}")
        .service(overwrite::overwrite)
        .service(write_new::write_new)
        .service(read::read)
        .service(mkdir::mkdir)
}
