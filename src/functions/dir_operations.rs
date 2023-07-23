use async_recursion::async_recursion;
use goodmorning_bindings::services::v1::{V1DirTreeItem, V1DirTreeNode};
use std::path::Path;
use std::{error::Error, time::UNIX_EPOCH};
use tokio::fs;

use crate::structs::{ItemVisibility, Visibilities, Visibility};

use super::is_bson;

#[async_recursion]
pub async fn copy_folder_owned(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst).await?;
    let mut entries = fs::read_dir(src).await?;
    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        if is_bson(&entry_path) {
            continue;
        }
        let dst_path = dst.join(entry.file_name());

        if entry.metadata().await?.is_dir() {
            copy_folder_owned(&entry_path, &dst_path).await?;
        } else {
            fs::copy(src, dst).await?;
        }
    }
    // if fs::metadata(src).await?.is_dir() {
    //
    // } else {
    //     fs::copy(src, dst).await?;
    // }

    Ok(())
}

#[async_recursion]
pub async fn copy_folder_unowned(
    src: &Path,
    dst: &Path,
    vis: ItemVisibility,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(dst).await?;
    let mut entries = fs::read_dir(src).await?;
    let visibilities = Visibilities::read_dir(src).await?;
    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        if is_bson(&entry_path) {
            continue;
        }
        let mut visibility = visibilities.get(entry_path.file_name().unwrap().to_str().unwrap());
        if visibility.inherited {
            visibility.visibility = vis;
        }

        if visibility.visibility != ItemVisibility::Public {
            continue;
        }

        let dst_path = dst.join(entry.file_name());

        if entry.metadata().await?.is_dir() {
            copy_folder_owned(&entry_path, &dst_path).await?;
        } else {
            fs::copy(src, dst).await?;
        }
    }

    Ok(())
}

#[async_recursion]
pub async fn dirtree(
    src: &Path,
    owned: bool,
    vis: Visibility,
) -> Result<V1DirTreeNode, Box<dyn Error>> {
    let mut entries = fs::read_dir(src).await?;
    let visibilities = Visibilities::read_dir(src).await?;

    let mut items = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        if is_bson(&entry.path()) {
            continue;
        }

        let name = entry.file_name().into_string().unwrap();
        let mut item_vis = visibilities.get(&name);
        if item_vis.inherited {
            item_vis.visibility = vis.visibility;
        }

        if !owned && item_vis.visibility != ItemVisibility::Public {
            continue;
        }

        let metadata = entry.metadata().await?;
        let item = if metadata.is_file() {
            V1DirTreeNode {
                visibility: item_vis.into(),
                name,
                content: V1DirTreeItem::File {
                    last_modified: metadata
                        .modified()
                        .unwrap_or(UNIX_EPOCH)
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    size: metadata.len(),
                },
            }
        } else {
            dirtree(&entry.path(), owned, item_vis).await?
        };

        items.push(item);
    }

    Ok(V1DirTreeNode {
        name: src.file_stem().unwrap().to_string_lossy().to_string(),
        visibility: vis.into(),
        content: V1DirTreeItem::Dir { content: items },
    })
}
