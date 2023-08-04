use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::functions::{bson_read, bson_write, get_usersys_dir};

use super::GMServices;

#[derive(Serialize, Deserialize, Default)]
pub struct TexCollection {
    pub name: String,
    pub description: String,
    pub items: Vec<TexCollectionItem>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TexCollectionItem {
    pub author: i64,
    pub id: u64,
}

pub struct TexPublishedItem {
    pub name: String,
    pub description: String,
}

impl TexCollection {
    pub async fn read(id: i64, collection_id: u64) -> Result<Self, Box<dyn Error>> {
        bson_read(
            &get_usersys_dir(id, Some(GMServices::Tex))
                .join("collections")
                .join(collection_id.to_string())
                .with_extension("bson"),
        )
        .await
    }

    pub async fn write(&self, id: i64, collection_id: u64) -> Result<(), Box<dyn Error>> {
        bson_write(
            &self,
            &get_usersys_dir(id, Some(GMServices::Tex))
                .join("collections")
                .join(collection_id.to_string())
                .with_extension("bson"),
        )
        .await
    }
}
