use actix_web::Scope;

mod copy;
mod copy_overwrite;
mod delete;
mod delete_multiple;
mod diritems;
mod exists;
mod file;
mod mkdir;
mod mkdir_multiple;
mod r#move;
mod move_overwrite;
mod move_createdirs;
mod remove_visibility;
mod set_visibility;
mod touch;
mod tree;
mod upload;
mod upload_createdirs;
mod upload_overwrite;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(upload_overwrite::upload_overwrite)
        .service(upload::upload)
        .service(upload_createdirs::upload_createdirs)
        .service(upload_createdirs::upload_createdirs_overwrite)
        .service(mkdir::mkdir)
        .service(mkdir_multiple::mkdir_multiple)
        .service(set_visibility::set_visibility)
        .service(remove_visibility::remove_visibility)
        .service(delete::delete)
        .service(delete_multiple::delete_multiple)
        .service(touch::touch)
        .service(copy::copy)
        .service(copy_overwrite::copy_overwrite)
        .service(r#move::r#move)
        .service(move_overwrite::r#move)
        .service(move_createdirs::r#move)
        .service(move_createdirs::move_overwrite)
        .service(file::file)
        .service(diritems::diritems)
        .service(tree::tree)
        .service(exists::exists)
}
