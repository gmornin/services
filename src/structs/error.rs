use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum GMError {
    UserNotFound,
    EmailMismatch,
    TriggerNotFound,
}

impl Display for GMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl Error for GMError {}
