use actix_web::{
    web::{Data, Json},
    *,
};
use mongodb::Database;
use std::{
    error::Error,
    ffi::OsStr,
    path::{Path as FilePath, PathBuf},
};
use tokio::{fs, io};

use goodmorning_bindings::{
    services::v1::{V1Error, V1FromTo, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*};

#[post("/copy")]
pub async fn copy(
    db: Data<Database>,
    post: Json<V1FromTo>,
    storage_limits: Data<StorageLimits>,
) -> Json<V1Response> {
    Json(V1Response::from_res(
        copy_task(&post.from, &post.to, &post.token, &db, &storage_limits).await,
    ))
}

async fn copy_task(
    from: &str,
    to: &str,
    token: &str,
    db: &Database,
    storage_limits: &StorageLimits,
) -> Result<V1Response, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(V1Response::Error {
                kind: V1Error::InvalidToken,
            })
        }
    };

    let path_buf = PathBuf::from(format!("usercontent/{}", account.id)).join(to);

    if !editable(&path_buf) {
        return Err(V1Error::NotEditable.into());
    }

    let from_buf = PathBuf::from(format!("usercontent/{}", from));

    if from_buf
        .iter()
        .any(|section| section == OsStr::new(".") || section == OsStr::new(".."))
    {
        return Err(V1Error::FileNotFound.into());
    }

    if !fs::try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    let user = match from_buf.iter().nth(1) {
        Some(id) => id.to_str().unwrap(),
        None => return Err(V1Error::FileNotFound.into()),
    };

    if user != account.id
        && (!fs::try_exists(&from_buf).await?
            || Visibilities::visibility(&from_buf).await?.visibility == ItemVisibility::Private)
    {
        return Err(V1Error::FileNotFound.into());
    }

    let metadata = fs::metadata(&from_buf).await?;
    println!("{:?}", path_buf);

    if metadata.is_file() {
        if account
            .exceeds_limit(storage_limits, Some(metadata.len()), None)
            .await?
        {
            return Err(V1Error::FileTooLarge.into());
        }
        remove(&path_buf).await?;
        fs::copy(&from_buf, &path_buf).await?;
    } else {
        if account
            .exceeds_limit(storage_limits, Some(dir_size(&from_buf).await?), None)
            .await?
        {
            return Err(V1Error::FileTooLarge.into());
        }
        remove(&path_buf).await?;
        copy_folder(&from_buf, &path_buf).await?;
    }

    let mut from_visibilities = Visibilities::read_dir(from_buf.parent().unwrap()).await?;
    let from_visibility = match from_visibilities
        .0
        .get_mut(from_buf.file_name().unwrap().to_str().unwrap())
    {
        Some(visibility) => *visibility,
        None => {
            return Ok(V1Response::Copied {
                path: format!("/{}/{}", account.id, to),
            })
        }
    };

    let file_name = path_buf
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    if from_buf.parent() == path_buf.parent() {
        let _ = from_visibilities.0.insert(file_name, from_visibility);
        from_visibilities.save(path_buf.parent().unwrap()).await?;
    } else {
        let mut new_visibilities = Visibilities::read_dir(path_buf.parent().unwrap()).await?;
        new_visibilities.0.insert(file_name, from_visibility);
        new_visibilities.save(path_buf.parent().unwrap()).await?;
    }

    Ok(V1Response::Copied {
        path: format!("/{}/{}", account.id, to),
    })
}

async fn remove(path: &FilePath) -> io::Result<()> {
    if fs::metadata(&path).await?.is_dir() {
        fs::remove_dir_all(&path).await
    } else {
        fs::remove_file(&path).await
    }
}
