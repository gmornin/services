use actix_web::{web::Json, *};
use goodmorning_bindings::services::v1::{V1Error, V1FromTo, V1Response};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use crate::{functions::*, structs::*, *};

#[post("/copy")]
pub async fn copy(post: Json<V1FromTo>) -> HttpResponse {
    from_res(copy_task(post).await)
}

async fn copy_task(post: Json<V1FromTo>) -> Result<V1Response, Box<dyn Error>> {
    let account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let user_tobuf = PathBuf::from(post.to.trim_start_matches('/'));
    let user_frombuf = PathBuf::from(post.from.trim_start_matches('/'));

    if !editable(&user_tobuf)
        || is_bson(&user_frombuf)
        || has_dotdot(&user_tobuf)
        || has_dotdot(&user_frombuf)
    {
        return Err(V1Error::PermissionDenied.into());
    }

    let to_buf = get_user_dir(account.id, None).join(user_tobuf);
    let from_buf = get_user_dir(account.id, None).join(user_frombuf);

    if fs::try_exists(&to_buf).await? {
        return Err(V1Error::PathOccupied.into());
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

    if from_buf.extension() != to_buf.extension() {
        return Err(V1Error::ExtensionMismatch.into());
    }

    let metadata = fs::metadata(&from_buf).await?;

    if metadata.is_file() {
        if account
            .exceeds_limit(STORAGE_LIMITS.get().unwrap(), Some(metadata.len()), None)
            .await?
        {
            return Err(V1Error::FileTooLarge.into());
        }
        fs::copy(&from_buf, &to_buf).await?;
    } else {
        return Err(V1Error::PermissionDenied.into());
        // if account
        //     .exceeds_limit(
        //         STORAGE_LIMITS.get().unwrap(),
        //         Some(dir_size(&from_buf).await?),
        //         None,
        //     )
        //     .await?
        // {
        //     return Err(V1Error::FileTooLarge.into());
        // }
        // copy_folder(&from_buf, &to_buf).await?;
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
