use std::error::Error;
use std::path::Path;

use async_recursion::async_recursion;
use tokio::{fs, io};

use crate::structs::{ItemVisibility, Visibilities};

#[async_recursion]
pub async fn dir_size(path: &Path) -> io::Result<u64> {
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

#[async_recursion]
pub async fn dir_size_unowned(path: &Path, vis: ItemVisibility) -> Result<u64, Box<dyn Error>> {
    let mut running_total = 0;
    let mut dir = fs::read_dir(path).await?;
    let visibilities = Visibilities::read_dir(path).await?;
    while let Some(item) = dir.next_entry().await? {
        let mut visibility = visibilities.get(item.file_name().as_os_str().to_str().unwrap());
        if visibility.inherited {
            visibility.visibility = vis;
        }

        if visibility.visibility != ItemVisibility::Public {
            continue;
        }
        let metadata = item.metadata().await?;
        if metadata.is_dir() {
            let add = dir_size_unowned(&item.path(), visibility.visibility).await?;
            running_total += add;
        } else {
            running_total += metadata.len();
        }
    }

    Ok(running_total)
}

pub async fn file_size(path: &Path) -> io::Result<u64> {
    Ok(fs::metadata(path).await?.len())
}
