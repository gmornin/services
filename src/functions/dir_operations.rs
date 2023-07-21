// code by sage from poe.com idk what this does

use async_recursion::async_recursion;
use std::path::Path;
use tokio::fs;

use crate::structs::ItemVisibility;

#[async_recursion]
pub async fn copy_folder(
    src: &Path,
    dst: &Path,
    vis: Option<ItemVisibility>,
) -> std::io::Result<()> {
    if src.is_dir() {
        match vis {
            Some(_vis) => {
                todo!()
            }
            None => {
                fs::create_dir_all(dst).await?;

                let mut entries = fs::read_dir(src).await?;
                while let Some(entry) = entries.next_entry().await? {
                    let entry_path = entry.path();
                    let dst_path = dst.join(entry.file_name());

                    if entry_path.is_dir() {
                        copy_folder(&entry_path, &dst_path, None).await?;
                    } else {
                        fs::copy(&entry_path, &dst_path).await?;
                    }
                }
            }
        }
    } else {
        fs::copy(src, dst).await?;
    }

    Ok(())
}
