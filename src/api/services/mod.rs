pub mod v1;

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/services")
        .service(v1::scope())
}
