use actix_web::{web::Json, *};
use std::{
    error::Error,
    path::{Path as FilePath, PathBuf},
};
use tokio::{fs, io};

use goodmorning_bindings::services::v1::{V1Error, V1FromTo, V1Response};

use crate::{functions::*, structs::*, *};

#[post("/copy-overwrite")]
pub async fn copy_overwrite(post: Json<V1FromTo>) -> HttpResponse {
    from_res(copy_overwrite_task(post).await)
}

async fn copy_overwrite_task(post: Json<V1FromTo>) -> Result<V1Response, Box<dyn Error>> {
    let accounts = get_accounts(DATABASE.get().unwrap());
    let account = match Account::find_by_token(&post.token, &accounts).await? {
        Some(account) => account,
        None => {
            return Ok(V1Response::Error {
                kind: V1Error::InvalidToken,
            })
        }
    };

    if !account.verified {
        return Ok(V1Response::Error {
            kind: V1Error::NotVerified,
        });
    }

    let to_buf = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(account.id.to_string())
        .join(post.to.trim_start_matches('/'));
    let from_buf = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(post.from_userid.to_string())
        .join(post.from.trim_start_matches('/'));

    if !editable(&to_buf) || is_bson(&from_buf) || has_dotdot(&to_buf) || has_dotdot(&from_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if post.from_userid == account.id {
        if !fs::try_exists(&from_buf).await? {
            return Err(V1Error::FileNotFound.into());
        }
    } else if !fs::try_exists(&from_buf).await?
        || Visibilities::visibility(&from_buf).await?.visibility == ItemVisibility::Private
    {
        return Err(V1Error::FileNotFound.into());
    }

    let metadata = fs::metadata(&from_buf).await?;

    if metadata.is_file() {
        if account
            .exceeds_limit(STORAGE_LIMITS.get().unwrap(), Some(metadata.len()), None)
            .await?
        {
            return Err(V1Error::FileTooLarge.into());
        }
        remove(&to_buf).await?;
        fs::copy(&from_buf, &to_buf).await?;
    } else {
        if account
            .exceeds_limit(
                STORAGE_LIMITS.get().unwrap(),
                Some(dir_size(&from_buf).await?),
                None,
            )
            .await?
        {
            return Err(V1Error::FileTooLarge.into());
        }
        remove(&to_buf).await?;
        copy_folder(&from_buf, &to_buf).await?;
    }

    let mut from_visibilities = Visibilities::read_dir(from_buf.parent().unwrap()).await?;
    let from_visibility = match from_visibilities
        .0
        .get_mut(from_buf.file_name().unwrap().to_str().unwrap())
    {
        Some(visibility) => *visibility,
        None => return Ok(V1Response::Copied),
    };

    let file_name = to_buf
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    if from_buf.parent() == to_buf.parent() {
        let _ = from_visibilities.0.insert(file_name, from_visibility);
        from_visibilities.save(to_buf.parent().unwrap()).await?;
    } else {
        let mut new_visibilities = Visibilities::read_dir(to_buf.parent().unwrap()).await?;
        new_visibilities.0.insert(file_name, from_visibility);
        new_visibilities.save(to_buf.parent().unwrap()).await?;
    }

    Ok(V1Response::Copied)
}

async fn remove(path: &FilePath) -> io::Result<()> {
    if fs::try_exists(path).await? {
        return Ok(());
    }
    if fs::metadata(&path).await?.is_dir() {
        fs::remove_dir_all(&path).await
    } else {
        fs::remove_file(&path).await
    }
}
