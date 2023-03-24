mod r#use;

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/trigger").service(r#use::r#use)
}
