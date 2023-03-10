use std::{error::Error, fmt};

use actix_web::Scope;
use serde::{Deserialize, Serialize};

mod create;
mod delete;
mod gettoken;
mod regeneratetoken;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Responses {
    #[serde(rename = "error")]
    Error { kind: ErrorKind },
    #[serde(rename = "created")]
    Created { id: String },
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "token")]
    GetToken { token: String },
    #[serde(rename = "regnerated")]
    RegenerateToken { token: String },
}

#[derive(Serialize, Deserialize, Debug)]
enum ErrorKind {
    #[serde(rename = "username taken")]
    UsernameTaken,
    #[serde(rename = "email taken")]
    EmailTaken,
    #[serde(rename = "no such user")]
    NoSuchUser,
    #[serde(rename = "password incorrect")]
    PasswordIncorrect,
    #[serde(rename = "invalid token")]
    InvalidToken,
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
    Scope::new("/account")
        .service(create::create)
        .service(delete::delete)
        .service(gettoken::get_token)
        .service(regeneratetoken::regenerate_token)
}
