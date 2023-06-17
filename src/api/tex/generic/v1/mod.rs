use actix_web::Scope;

mod create;
mod pfp;
mod profile;
mod set_pfp;
mod set_profile;

pub fn scope() -> Scope {
    Scope::new("v1")
        .service(create::create)
        .service(set_profile::set_profile)
        .service(set_pfp::set_pfp)
        .service(profile::profile)
        .service(pfp::pfp)
}
