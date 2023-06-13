use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GMServices {
    #[serde(rename = "tex")]
    Tex,
}
