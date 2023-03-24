use serde::Serialize;

use crate::traits::ResTrait;

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

    // trigger
    #[serde(rename = "triggered")]
    Triggered,

    // usercontent
    #[serde(rename = "overwritten")]
    Overwritten { path: String },

    #[serde(rename = "error")]
    Error { kind: GMError },
}

impl ResTrait for GMResponses {
    type Error = GMError;

    fn error(e: <GMResponses as ResTrait>::Error) -> Self {
        Self::Error { kind: e }
    }
}
