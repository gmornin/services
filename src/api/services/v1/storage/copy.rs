use actix_web::{
    web::{Data, Json, Path},
    *,
};
use mongodb::Database;
use serde::Deserialize;
use tokio::fs;
use std::{error::Error, path::PathBuf, ffi::OsStr};


use crate::{
    api::services::v1::*,
    functions::*,
    structs::*,
};

#[derive(Deserialize)]
struct StaticPath {
    path: String,
    token: String,
}

#[derive(Deserialize)]
struct CopyFrom {
    pub from: String,
}

#[post("/copy/{path:.*}")]
pub async fn copy(
    path: Path<StaticPath>,
    db: Data<Database>,
    post: Json<CopyFrom>,
    storage_limits: Data<StorageLimits>,
) -> Json<GMResponses> {
    Json(to_res(
        copy_task(&path.path, &path.token, &db, &post, &storage_limits).await,
    ))
}

async fn copy_task(
    path: &str,
    token: &str,
    db: &Database,
    post: &CopyFrom,
    storage_limits: &StorageLimits,
) -> Result<GMResponses, Box<dyn Error>> {
    let accounts = get_accounts(db);
    let account = match Account::find_by_token(token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(GMResponses::Error {
                kind: GMError::InvalidToken,
            })
        }
    };

    let path_buf = PathBuf::from(format!("usercontent/{}", account.id)).join(path);

    if !editable(&path_buf) {
        return Err(GMError::NotEditable.into());
    }

    let from_buf = PathBuf::from(format!("usercontent/{}", post.from));

    if from_buf.iter().any(|section| section == OsStr::new(".") || section == OsStr::new("..")) {
        return Err(GMError::FileNotFound.into());
    }

    if fs::try_exists(&path_buf).await? {
        return Err(GMError::PathOccupied.into());
    }

    let user = match from_buf.iter().nth(1) {
        Some(id) => id.to_str().unwrap(),
        None => return Err(GMError::FileNotFound.into())
    };

    if user != account.id && (!fs::try_exists(&from_buf).await? || Visibilities::visibility(&from_buf).await?.visibility == ItemVisibility::Private) {
        return Err(GMError::FileNotFound.into());
    }
    
    let metadata = fs::metadata(&from_buf).await?;
    println!("{:?}", path_buf);

    if metadata.is_file() {
        if account.exceeds_limit(storage_limits, Some(metadata.len()), None).await? {
            return Err(GMError::FileTooLarge.into());
        }
        fs::copy(from_buf, path_buf).await?;
    } else {
        if account.exceeds_limit(storage_limits, Some(dir_size(&from_buf).await?), None).await? {
            return Err(GMError::FileTooLarge.into());
        }
        copy_folder(&from_buf, &path_buf).await?;
    }
    

    Ok(GMResponses::Copied { path: format!("/{}/{}", account.id, path) })
}
