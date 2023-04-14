use actix_web::Scope;

mod remove_visibility;
mod set_visibility;
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
        .service(set_visibility::set_visibility)
        .service(remove_visibility::remove_visibility)
}
