use actix_web::Scope;

mod create;
mod delete;
mod login;
mod regeneratetoken;
mod rename;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(create::create)
        .service(delete::delete)
        .service(login::login)
        .service(regeneratetoken::regenerate_token)
        .service(rename::rename)
}
