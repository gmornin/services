use actix_web::Scope;

mod overwrite;

pub fn scope() -> Scope {
    Scope::new("/storage/{token}").service(overwrite::overwrite)
}
