use actix_web::Scope;

mod copy;
mod copy_overwrite;
mod delete;
mod diritems;
mod file;
mod mkdir;
mod r#move;
mod move_overwrite;
mod remove_visibility;
mod set_visibility;
mod touch;
mod tree;
mod upload;
mod upload_overwrite;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(upload_overwrite::upload_overwrite)
        .service(upload::upload)
        .service(mkdir::mkdir)
        .service(set_visibility::set_visibility)
        .service(remove_visibility::remove_visibility)
        .service(delete::delete)
        .service(touch::touch)
        .service(copy::copy)
        .service(r#move::r#move)
        .service(file::file)
        .service(diritems::diritems)
        .service(tree::tree)
}
