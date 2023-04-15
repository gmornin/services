use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{structs::Visibility, traits::ResTrait};

use super::error::GMError;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum GMResponses {
    // account
    #[serde(rename = "created")]
    Created { id: String, token: String },
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "token")]
    GetToken { token: String },
    #[serde(rename = "regnerated")]
    RegenerateToken { token: String },
    #[serde (rename = "renamed")]
    Renamed,

    // trigger
    #[serde(rename = "triggered")]
    Triggered,

    // storage
    #[serde(rename = "overwritten")]
    Overwritten { path: String },
    #[serde(rename = "dir content")]
    DirContent(HashMap<String, DirItem>),
    #[serde(rename = "visibility changed")]
    VisibilityChanged,
    #[serde(rename = "file item created")]
    FileItemCreated { path: String },
    #[serde(rename = "file item deleted")]
    FileItemDeleted,
    #[serde(rename = "copied")]
    Copied { path: String },
    #[serde(rename = "moved")]
    Moved { path: String },

    #[serde(rename = "nothing changed")]
    NothingChanged,

    #[serde(rename = "error")]
    Error { kind: GMError },
}

impl ResTrait for GMResponses {
    type Error = GMError;

    fn error(e: <GMResponses as ResTrait>::Error) -> Self {
        Self::Error { kind: e }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DirItem {
    pub visibility: Visibility,
    pub is_file: bool,
    pub name: String,
}
