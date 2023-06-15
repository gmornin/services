use actix_web::Scope;

mod create;
mod profile;
mod set_profile;
mod set_profile_image;

pub fn scope() -> Scope {
    Scope::new("v1")
        .service(create::create)
        .service(set_profile::set_profile)
        .service(set_profile_image::set_profile_image)
        .service(profile::profile)
}
