use actix_web::cookie::Cookie;

pub fn cookie_to_str<'c>(cookie: &'c Option<Cookie<'static>>) -> Option<&'c str> {
    match cookie {
        Some(cookie) => Some(cookie.value()),
        None => None,
    }
}
