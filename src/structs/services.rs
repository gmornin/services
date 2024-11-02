use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum GMServices {
    #[cfg(feature = "tex")]
    #[serde(rename = "tex")]
    Tex,
    #[cfg(feature = "blue")]
    #[serde(rename = "blue")]
    Blue,
    #[serde(other)]
    Other,
}

impl GMServices {
    pub fn as_str(&self) -> &str {
        match self {
            #[cfg(feature = "tex")]
            Self::Tex => "tex",
            #[cfg(feature = "blue")]
            Self::Blue => "blue",
            Self::Other => "other",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            #[cfg(feature = "tex")]
            "tex" => Some(Self::Tex),
            #[cfg(feature = "blue")]
            "Blue" => Some(Self::Blue),
            _ => None,
        }
    }
}
