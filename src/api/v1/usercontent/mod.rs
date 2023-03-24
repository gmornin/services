use actix_web::Scope;

mod overwrite;

pub fn scope() -> Scope {
    Scope::new("/usercontent").service(overwrite::overwrite)
}
