use actix_web::Scope;

mod trigger;

pub fn scope() -> Scope {
    Scope::new("").service(trigger::trigger)
}
