use actix_web::HttpResponse;
use goodmorning_bindings::traits::SerdeAny;

pub trait Peekable: SerdeAny {
    fn to_html(&self) -> Option<HttpResponse> {
        None
    }
}
