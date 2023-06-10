use actix_web::Scope;

mod main;

pub fn scope() -> Scope {
    Scope::new("/usercontent").service(main::by_id)
}
