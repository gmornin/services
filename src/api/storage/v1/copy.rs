use actix_web::{web::Json, *};
use goodmorning_bindings::services::v1::{V1Error, V1FromTo, V1Response};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use crate::{functions::*, structs::*, traits::CollectionItem, *};

#[post("/copy")]
pub async fn copy(post: Json<V1FromTo>) -> HttpResponse {
    from_res(copy_task(post).await)
}

async fn copy_task(post: Json<V1FromTo>) -> Result<V1Response, Box<dyn Error>> {
    let mut account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let user_tobuf = PathBuf::from(post.to.trim_start_matches('/'));
    let user_frombuf = PathBuf::from(post.from.trim_start_matches('/'));

    if !editable(&user_tobuf, &account.services)
        || is_bson(&user_frombuf)
        || has_dotdot(&user_tobuf)
        || has_dotdot(&user_frombuf)
    {
        return Err(V1Error::PermissionDenied.into());
    }

    let to_buf = get_user_dir(account.id, None).join(user_tobuf);
    let from_buf = get_user_dir(post.from_userid, None).join(user_frombuf);

    if fs::try_exists(&to_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    let mut vis = None;

    if post.from_userid == account.id {
        if !fs::try_exists(&from_buf).await? {
            return Err(V1Error::FileNotFound.into());
        }
    } else {
        vis = Some(Visibilities::visibility(&from_buf).await?.visibility);
        if !fs::try_exists(&from_buf).await? || vis == Some(ItemVisibility::Private) {
            return Err(V1Error::FileNotFound.into());
        }
    }

    let metadata = fs::metadata(&from_buf).await?;

    if metadata.is_file() {
        if from_buf.extension() != to_buf.extension() {
            return Err(V1Error::ExtensionMismatch.into());
        }

        if account
            .exceeds_limit_nosave(STORAGE_LIMITS.get().unwrap(), Some(metadata.len()), None)
            .await?
        {
            account.save_replace(ACCOUNTS.get().unwrap()).await?;
            return Err(V1Error::StorageFull.into());
        }
        fs::copy(&from_buf, &to_buf).await?;
        account.stored.as_mut().unwrap().value += metadata.len();
    } else if account.id == post.from_userid {
        let size = dir_size(&from_buf).await?;
        if account
            .exceeds_limit(STORAGE_LIMITS.get().unwrap(), Some(size), None)
            .await?
        {
            return Err(V1Error::StorageFull.into());
        }
        copy_folder_owned(&from_buf, &to_buf).await?;
        account.stored.as_mut().unwrap().value += size;
    } else {
        let size = dir_size_unowned(&from_buf, vis.unwrap()).await?;
        if account
            .exceeds_limit(STORAGE_LIMITS.get().unwrap(), Some(size), None)
            .await?
        {
            return Err(V1Error::StorageFull.into());
        }
        copy_folder_unowned(&from_buf, &to_buf, vis.unwrap()).await?;
        account.stored.as_mut().unwrap().value += size;
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

    account.save_replace(ACCOUNTS.get().unwrap()).await?;

    Ok(V1Response::Copied)
}
