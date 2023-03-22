use actix_web::Scope;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

mod upload;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Responses {
    #[serde(rename = "overwritten")]
    Overwritten { path: String },
    #[serde(rename = "error")]
    Error { kind: ErrorKind },
}

#[derive(Serialize, Deserialize, Debug)]
enum ErrorKind {
    #[serde(rename = "invalid token")]
    InvalidToken,
    #[serde(rename = "path occupied")]
    PathOccupied,
    #[serde(rename = "not found")]
    NotFound,
    #[serde(rename = "filesystem error")]
    FsError(String),
    #[serde(rename = "external")]
    External(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl Error for ErrorKind {}

pub fn scope() -> Scope {
    Scope::new("/usercontent").service(upload::overwrite)
}
