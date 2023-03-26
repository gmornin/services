use std::path::PathBuf;

use async_recursion::async_recursion;
use tokio::{fs, io};

#[async_recursion]
pub async fn dir_size(path: &PathBuf) -> io::Result<u64> {
    let mut running_total = 0;
    let mut dir = fs::read_dir(path).await?;
    while let Some(item) = dir.next_entry().await? {
        let metadata = item.metadata().await?;
        if metadata.is_dir() {
            running_total += dir_size(&item.path()).await?;
        } else {
            running_total += metadata.len();
        }
    }

    Ok(running_total)
}

pub async fn file_size(path: &PathBuf) -> io::Result<u64> {
    Ok(fs::metadata(path).await?.len())
}
