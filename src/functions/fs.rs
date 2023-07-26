use std::{error::Error, path::Path, time::UNIX_EPOCH};

use goodmorning_bindings::services::v1::*;
use tokio::fs::{self, try_exists};

use crate::{
    functions::{has_dotdot, is_bson},
    structs::Visibilities,
};

use super::get_user_dir;

pub async fn dir_items(
    id: i64,
    path: &Path,
    is_owner: bool,
    skip_dir_check: bool,
) -> Result<Vec<V1DirItem>, Box<dyn Error>> {
    if path.has_root() {
        return Err(V1Error::PermissionDenied.into());
    }

    let pathbuf = get_user_dir(id, None).join(path);

    if !try_exists(&pathbuf).await? | has_dotdot(&pathbuf) | is_bson(&pathbuf) {
        return Err(V1Error::FileNotFound.into());
    }
    let mut items = Vec::new();

    if is_owner {
        if !skip_dir_check && !fs::metadata(&pathbuf).await?.is_dir() {
            return Err(V1Error::TypeMismatch.into());
        }

        let dir_visibilily = Visibilities::visibility(&pathbuf).await?;
        let items_visibilities = Visibilities::read_dir(&pathbuf).await?;
        let mut dir_content = fs::read_dir(&pathbuf).await?;

        while let Some(entry) = dir_content.next_entry().await? {
            if is_bson(&entry.path()) {
                continue;
            }

            let metadata = entry.metadata().await?;

            items.push(V1DirItem {
                name: entry.file_name().to_os_string().into_string().unwrap(),
                is_file: metadata.is_file(),
                visibility: items_visibilities
                    .get(entry.file_name().to_str().unwrap())
                    .overwrite_if_inherited(dir_visibilily)
                    .into(),
                last_modified: metadata
                    .modified()?
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                size: metadata.len(),
            });
        }
    } else {
        let dir_visibilily = Visibilities::visibility(&pathbuf).await?;

        if dir_visibilily.visibility == crate::structs::ItemVisibility::Private {
            return Err(V1Error::FileNotFound.into());
        }

        if !skip_dir_check && !fs::metadata(&pathbuf).await?.is_dir() {
            return Err(V1Error::TypeMismatch.into());
        }

        let items_visibilities = Visibilities::read_dir(&pathbuf).await?;
        let mut dir_content = fs::read_dir(&pathbuf).await?;

        while let Some(entry) = dir_content.next_entry().await? {
            if is_bson(&entry.path()) {
                continue;
            }

            let metadata = entry.metadata().await?;
            let visibility: V1Visibility = items_visibilities
                .get(entry.file_name().to_str().unwrap())
                .overwrite_if_inherited(dir_visibilily)
                .into();

            if !matches!(visibility.visibility, ItemVisibility::Public) {
                continue;
            }

            items.push(V1DirItem {
                name: entry.file_name().to_os_string().into_string().unwrap(),
                is_file: metadata.is_file(),
                visibility,
                last_modified: metadata
                    .modified()?
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                size: metadata.len(),
            });
        }
    }

    Ok(items)
}
