use std::{error::Error, ffi::OsStr, path::Path, time::UNIX_EPOCH};

use futures_util::StreamExt;
use goodmorning_bindings::services::v1::*;
use mongodb::bson::doc;
use tokio::fs::{self, try_exists};

use crate::{
    functions::{has_dotdot, is_bson},
    structs::{Account, Visibilities},
    ACCOUNTS,
};

use super::get_user_dir;

pub async fn dir_items(
    mut id: i64,
    path: &Path,
    is_owner: bool,
    skip_dir_check: bool,
) -> Result<Vec<V1DirItem>, Box<dyn Error>> {
    if path.has_root() {
        return Err(V1Error::PermissionDenied.into());
    }

    let mut path = path.to_path_buf();

    let mut items = Vec::new();

    if path.iter().count() == 1 {
        items.push(V1DirItem {
            visibility: V1Visibility {
                inherited: true,
                visibility: ItemVisibility::Private,
            },
            is_file: false,
            name: "Shared".to_string(),
            last_modified: 0,
            size: 0,
        });
    } else if path.iter().nth(1) == Some(OsStr::new("Shared")) {
        if path.iter().count() == 2 {
            let mut users = Vec::new();
            let mut cursor = ACCOUNTS
                .get()
                .unwrap()
                .find(doc! { "access.file": id, "services": path.iter().next().unwrap().to_str().unwrap() }, None)
                .await?;

            while let Some(user) = cursor.next().await {
                let mut user = user?;
                users.push(V1DirItem {
                    visibility: V1Visibility {
                        inherited: true,
                        visibility: ItemVisibility::Private,
                    },
                    is_file: false,
                    name: user.username.clone(),
                    last_modified: 0,
                    size: user.get_stored().await?,
                })
            }

            return Ok(users);
        } else {
            let target = Account::find_by_username(
                path.iter().nth(2).unwrap().to_string_lossy().to_string(),
            )
            .await?;

            if target.is_none()
                || !target
                    .as_ref()
                    .unwrap()
                    .access
                    .get(&AccessType::File)
                    .is_some_and(|set| set.contains(&id))
            {
                return Err(V1Error::FileNotFound.into());
            }

            id = target.unwrap().id;
            path = [path.iter().next().unwrap()]
                .into_iter()
                .chain(path.iter().skip(3))
                .collect();
        }
    }

    let pathbuf = get_user_dir(id, None).join(path);

    if !try_exists(&pathbuf).await? | has_dotdot(&pathbuf) | is_bson(&pathbuf) {
        return Err(V1Error::FileNotFound.into());
    }
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

    items.sort_by(|item1, item2| match (item1.is_file, item2.is_file) {
        (false, true) => std::cmp::Ordering::Less,
        (true, false) => std::cmp::Ordering::Greater,
        _ => item1.name.cmp(&item2.name),
    });
    Ok(items)
}
