mod peek;
mod revoke;
mod r#use;

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(r#use::r#use)
        .service(revoke::revoke)
        .service(peek::peek)
}
