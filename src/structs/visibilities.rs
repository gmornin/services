use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, path::Path};
use tokio::{fs::*, io::AsyncWriteExt};

pub const VISIBILITIES_BSON: &str = "visibility.bson";

#[derive(Default, Serialize, Deserialize)]
pub struct Visibilities(HashMap<String, Visibility>);

impl Visibilities {
    pub async fn check_all_dirs(path: &Path) -> Result<(), Box<dyn Error>> {
        let ancestors = path.ancestors().collect::<Vec<_>>();
        let take = ancestors.len() - 2;
        for path in ancestors.into_iter().take(take) {
            let path = path.join(VISIBILITIES_BSON);
            if !try_exists(&path).await? {
                let doc = bson::to_document(&Visibilities::default())?;
                let mut bytes = Vec::new();
                doc.to_writer(&mut bytes)?;
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .await?;
                file.write(&bytes).await?;
            }
        }

        Ok(())
    }

    pub async fn read_dir(path: &Path) -> Result<Visibilities, Box<dyn Error>> {
        let path = path.join(VISIBILITIES_BSON);
        let bytes = read(path).await?;
        let bson = bson::from_slice(&bytes)?;
        let visibilities: Visibilities = bson::from_bson(bson)?;

        Ok(visibilities)
    }

    pub async fn visibility(path: &Path) -> Result<Visibility, Box<dyn Error>> {
        Self::check_all_dirs(path.parent().unwrap()).await?;
        let ancestors = path.ancestors().collect::<Vec<_>>();
        let take = ancestors.len() - 3;
        for path in ancestors.into_iter().take(take) {
            println!("{}", path.as_os_str().to_str().unwrap());
            let dir_visibilily = Self::read_dir(path.parent().unwrap())
                .await?
                .get(path.file_name().unwrap().to_str().unwrap());
            if dir_visibilily != Visibility::Private {
                return Ok(dir_visibilily);
            }
        }

        Ok(Visibility::Private)
    }

    pub fn get(&self, path: &str) -> Visibility {
        self.0.get(path).copied().unwrap_or_default()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}

impl Visibility {
    pub fn overwrite_if_private(self, overwrite: &Visibility) -> Visibility {
        if self == Self::Private {
            *overwrite
        } else {
            self
        }
    }
}
