use actix_web::Scope;

mod remove_visibility;
mod set_visibility;
mod mkdir;
mod overwrite;
mod read;
mod write_new;
mod delete;
mod touch;
mod copy;
mod r#move;
mod move_overwrite;
mod copy_overwrite;

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
