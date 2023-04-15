// code by sage from poe.com idk what this does

use std::path::Path;
use async_recursion::async_recursion;
use tokio::fs;

#[async_recursion]
pub async fn copy_folder(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst).await?;

        let mut entries = fs::read_dir(src).await?;
        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if entry_path.is_dir() {
                copy_folder(&entry_path, &dst_path).await?;
            } else {
                fs::copy(&entry_path, &dst_path).await?;
            }
        }
    } else {
        fs::copy(src, dst).await?;
    }

    Ok(())
}
