use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum GMServices {
    #[serde(rename = "tex")]
    Tex,
}

impl GMServices {
    pub fn as_str(&self) -> &str {
        "tex"
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "tex" => Some(Self::Tex),
            _ => None,
        }
    }
}
