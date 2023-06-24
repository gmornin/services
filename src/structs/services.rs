use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum GMServices {
    #[serde(rename = "tex")]
    Tex,
}
