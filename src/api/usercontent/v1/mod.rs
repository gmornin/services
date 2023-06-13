use actix_web::Scope;

mod main;

pub fn scope() -> Scope {
    Scope::new("/v1").service(main::by_id)
}
