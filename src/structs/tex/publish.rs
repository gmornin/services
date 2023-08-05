use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{functions::get_tex_userpublishes, traits::CollectionItem};

#[derive(Serialize, Deserialize, Clone)]
pub struct TexPublish {
    #[serde(rename = "_id")]
    pub id: i64,
    pub published: u64,
    pub title: String,
    pub desc: String,
    pub ext: String,
}

impl CollectionItem<i64> for TexPublish {
    fn id(&self) -> i64 {
        self.id
    }
}

impl TexPublish {
    pub async fn new(
        userid: i64,
        publishid: &mut i64,
        title: String,
        desc: String,
        ext: String,
    ) -> Result<Self, Box<dyn Error>> {
        let s = Self {
            id: *publishid,
            published: chrono::Utc::now().timestamp() as u64,
            title,
            desc,
            ext,
        };

        s.save_create(&get_tex_userpublishes(userid)).await?;

        *publishid += 1;

        Ok(s)
    }
}
