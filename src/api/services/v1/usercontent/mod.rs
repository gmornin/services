use actix_web::Scope;

mod main;

pub fn scope() -> Scope {
    Scope::new("/").service(main::by_id)
}
