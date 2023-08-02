use actix_web::Scope;

mod jobs;
mod unqueue;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(jobs::jobs)
        .service(unqueue::unqueue)
}
