mod r#use;

use std::{fmt, error::Error};

use actix_web::Scope;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Responses {
    #[serde(rename = "error")]
    Error{ kind: ErrorKind },
    #[serde(rename = "triggered")]
    Triggered,
}

#[derive(Serialize, Deserialize, Debug)]
enum ErrorKind {
    #[serde(rename = "not found")]
    NotFound,
    #[serde(rename = "external")]
    External(String)
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl Error for ErrorKind {}

pub fn scope() -> Scope {
    Scope::new("/trigger")
        .service(r#use::r#use)
}
