use actix_web::Scope;

mod copy;
mod copy_overwrite;
mod delete;
mod mkdir;
mod r#move;
mod move_overwrite;
mod overwrite;
mod read;
mod remove_visibility;
mod set_visibility;
mod touch;
mod write_new;

pub fn scope() -> Scope {
    Scope::new("/storage/{token}")
        .service(overwrite::overwrite)
        .service(write_new::write_new)
        .service(read::read)
        .service(mkdir::mkdir)
        .service(set_visibility::set_visibility)
        .service(remove_visibility::remove_visibility)
        .service(delete::delete)
        .service(touch::touch)
        .service(copy::copy)
        .service(r#move::r#move)
}
