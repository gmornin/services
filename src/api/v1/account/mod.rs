use actix_web::Scope;

mod create;
mod delete;
mod gettoken;
mod regeneratetoken;
mod rename;



pub fn scope() -> Scope {
    Scope::new("/account")
        .service(create::create)
        .service(delete::delete)
        .service(gettoken::get_token)
        .service(regeneratetoken::regenerate_token)
        .service(rename::rename)
}
