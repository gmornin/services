use goodmorning_bindings::services::v1::{ItemVisibility as V1ItemVisibility, V1Visibility};
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, path::Path};
use tokio::{fs, io::AsyncWriteExt};

use crate::VIS_DEFAULT;

pub const VISIBILITIES_BSON: &str = "visibility.bson";

#[derive(Default, Serialize, Deserialize)]
pub struct Visibilities(pub HashMap<String, ItemVisibility>);

impl Visibilities {
    pub async fn check_all_dirs(path: &Path) -> Result<(), Box<dyn Error>> {
        let ancestors = path.ancestors().collect::<Vec<_>>();
        let take = ancestors.len() - 2;
        for path in ancestors.into_iter().take(take) {
            let path = path.join(VISIBILITIES_BSON);
            if !fs::try_exists(&path).await? {
                let doc = bson::to_document(&Visibilities::default())?;
                let mut bytes = Vec::new();
                doc.to_writer(&mut bytes)?;
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .await?;
                file.write_all(&bytes).await?;
            }
        }

        Ok(())
    }

    pub async fn read_dir(path: &Path) -> Result<Visibilities, Box<dyn Error>> {
        let path = path.join(VISIBILITIES_BSON);
        if !fs::try_exists(&path).await? {
            return Ok(Visibilities::default());
        }

        let bytes = fs::read(path).await?;
        let bson = bson::from_slice(&bytes)?;
        let visibilities: Visibilities = bson::from_bson(bson)?;

        Ok(visibilities)
    }

    pub async fn visibility(path: &Path) -> Result<Visibility, Box<dyn Error>> {
        let ancestors = path.ancestors().collect::<Vec<_>>();
        let take = ancestors.len() - 3;
        for (index, path) in ancestors.into_iter().take(take).enumerate() {
            let mut dir_visibilily = Self::read_dir(path.parent().unwrap())
                .await?
                .get(path.file_name().unwrap().to_str().unwrap());
            if !dir_visibilily.inherited {
                if index != 0 {
                    dir_visibilily.inherited = true;
                }
                return Ok(dir_visibilily);
            }
        }

        Ok(Visibility::default())
    }

    pub fn get(&self, path: &str) -> Visibility {
        match self.0.get(path).copied() {
            Some(visibility) => Visibility {
                inherited: false,
                visibility,
            },
            None => Visibility::default(),
        }
    }

    pub async fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let path = path.join(VISIBILITIES_BSON);
        let doc = bson::to_document(&self)?;
        let mut bytes = Vec::new();
        doc.to_writer(&mut bytes)?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .await?;
        file.write_all(&bytes).await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Visibility {
    pub inherited: bool,
    pub visibility: ItemVisibility,
}
impl From<Visibility> for V1Visibility {
    fn from(value: Visibility) -> Self {
        Self {
            inherited: value.inherited,
            visibility: value.visibility.into(),
        }
    }
}

impl Visibility {
    pub fn overwrite_if_inherited(mut self, dir_visibilily: Self) -> Self {
        if self.inherited {
            self.visibility = dir_visibilily.visibility;
        }

        self
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            inherited: true,
            visibility: ItemVisibility::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemVisibility {
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl From<V1ItemVisibility> for ItemVisibility {
    fn from(value: V1ItemVisibility) -> Self {
        match value {
            V1ItemVisibility::Hidden => Self::Hidden,
            V1ItemVisibility::Public => Self::Public,
            V1ItemVisibility::Private => Self::Private,
        }
    }
}

impl From<ItemVisibility> for V1ItemVisibility {
    fn from(value: ItemVisibility) -> Self {
        match value {
            ItemVisibility::Hidden => Self::Hidden,
            ItemVisibility::Public => Self::Public,
            ItemVisibility::Private => Self::Private,
        }
    }
}

impl Default for ItemVisibility {
    fn default() -> Self {
        *VIS_DEFAULT.get().unwrap()
    }
}
