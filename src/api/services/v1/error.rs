use std::{error::Error, fmt::Display};

use serde::Serialize;

use crate::traits::ErrorTrait;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum GMError {
    // accounts
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

    // triggers
    #[serde(rename = "email mismatch")]
    EmailMismatch,
    #[serde(rename = "trigger not found")]
    TriggerNotFound,

    // storage
    #[serde(rename = "path occupied")]
    PathOccupied,
    #[serde(rename = "file not found")]
    FileNotFound,
    #[serde(rename = "filesystem error")]
    FsError { content: String },
    #[serde(rename = "file too large")]
    FileTooLarge,
    #[serde(rename = "no parent")]
    NoParent,
    #[serde(rename = "not editable")]
    NotEditable,

    #[serde(rename = "external")]
    External { content: String },
}

impl ErrorTrait for GMError {
    fn external(e: Box<dyn Error>) -> Self {
        Self::External {
            content: e.to_string(),
        }
    }
}

impl Display for GMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl Error for GMError {}
