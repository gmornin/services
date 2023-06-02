use actix_web::{
    web::{Data, Json},
    *,
};
use mongodb::Database;
use std::{error::Error, ffi::OsStr, path::PathBuf};
use tokio::fs;

use goodmorning_bindings::{
    services::v1::{V1Error, V1FromTo, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*};

#[post("/move/{path:.*}")]
pub async fn r#move(db: Data<Database>, post: Json<V1FromTo>) -> Json<V1Response> {
    Json(V1Response::from_res(
        move_overwrite_task(&post.from, &post.to, &post.token, &db).await,
    ))
}

async fn move_overwrite_task(
    from: &str,
    to: &str,
    token: &str,
    db: &Database,
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

    let from_buf = PathBuf::from(format!("usercontent/{}/{}", account.id, from));

    if from_buf
        .iter()
        .any(|section| section == OsStr::new(".") || section == OsStr::new(".."))
    {
        return Err(V1Error::FileNotFound.into());
    }

    if !editable(&from_buf) {
        return Err(V1Error::NotEditable.into());
    }

    if !(fs::try_exists(&from_buf).await? && fs::try_exists(&path_buf).await?) {
        return Err(V1Error::FileNotFound.into());
    }

    if fs::metadata(&path_buf).await?.is_dir() {
        fs::remove_dir_all(&path_buf).await?;
    } else {
        fs::remove_file(&path_buf).await?;
    }

    fs::rename(&from_buf, &path_buf).await?;

    let mut from_visibilities = Visibilities::read_dir(from_buf.parent().unwrap()).await?;
    let from_visibility = match from_visibilities
        .0
        .get_mut(from_buf.file_name().unwrap().to_str().unwrap())
    {
        Some(visibility) => *visibility,
        None => {
            return Ok(V1Response::Moved {
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
        from_visibilities.0.remove(&file_name);
        from_visibilities.0.insert(file_name, from_visibility);
        from_visibilities.save(path_buf.parent().unwrap()).await?;
    } else {
        from_visibilities.0.remove(&file_name);
        let mut new_visibilities = Visibilities::read_dir(path_buf.parent().unwrap()).await?;
        new_visibilities.0.insert(file_name, from_visibility);
        new_visibilities.save(path_buf.parent().unwrap()).await?;
    }

    Ok(V1Response::Moved {
        path: format!("/{}/{}", account.id, to),
    })
}
