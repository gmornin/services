use std::error::Error;

use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder};
use goodmorning_bindings::traits::ResTrait;
use serde::Serialize;

pub fn from_res<R: ResTrait + Serialize>(res: Result<R, Box<dyn Error>>) -> HttpResponse {
    let res = R::from_res(res);
    HttpResponseBuilder::new(StatusCode::from_u16(res.status_code()).unwrap()).json(res)
}
