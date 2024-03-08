use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use goodmorning_bindings::services::v1::{V1Error, V1Response, V1SelfFromTo};

use crate::{functions::*, structs::*, traits::CollectionItem, ACCOUNTS};

#[post("/move-overwrite")]
pub async fn r#move(post: Json<V1SelfFromTo>) -> HttpResponse {
    from_res(move_overwrite_task(post).await)
}

async fn move_overwrite_task(post: Json<V1SelfFromTo>) -> Result<V1Response, Box<dyn Error>> {
    let mut account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let user_frombuf = PathBuf::from(post.from.trim_start_matches('/'));
    let user_tobuf = PathBuf::from(post.to.trim_start_matches('/'));

    if !editable(&user_tobuf, &account.services)
        || !editable(&user_frombuf, &account.services)
        || has_dotdot(&user_tobuf)
        || has_dotdot(&user_frombuf)
    {
        return Err(V1Error::PermissionDenied.into());
    }

    let to_buf = get_user_dir(account.id, None).join(user_tobuf);
    let from_buf = get_user_dir(account.id, None).join(user_frombuf);

    if !fs::try_exists(&from_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if !fs::try_exists(&to_buf.parent().unwrap()).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if from_buf.extension() != to_buf.extension() {
        return Err(V1Error::ExtensionMismatch.into());
    }

    let to_metedata = if fs::try_exists(&to_buf).await? {
        Some(fs::metadata(&to_buf).await?)
    } else {
        None
    };

    let to_size = match &to_metedata {
        Some(meta) if meta.is_file() => meta.len(),
        Some(_) => dir_size(&to_buf).await?,
        None => 0,
    };

    if let Some(meta) = to_metedata {
        if meta.is_dir() {
            fs::remove_dir_all(&to_buf).await?;
        } else {
            fs::remove_file(&to_buf).await?;
        }

        if to_size != 0 {
            if let Some(stored) = account.stored.as_mut() {
                stored.value = stored.value.saturating_sub(to_size);
                account.save_replace(ACCOUNTS.get().unwrap()).await?;
            }
        }
    }

    fs::rename(&from_buf, &to_buf).await?;

    let mut from_visibilities = Visibilities::read_dir(from_buf.parent().unwrap()).await?;
    let from_visibility = match from_visibilities
        .0
        .get_mut(from_buf.file_name().unwrap().to_str().unwrap())
    {
        Some(visibility) => *visibility,
        None => return Ok(V1Response::Moved),
    };

    let file_name = to_buf
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    if from_buf.parent() == to_buf.parent() {
        from_visibilities.0.remove(&file_name);
        from_visibilities.0.insert(file_name, from_visibility);
        from_visibilities.save(to_buf.parent().unwrap()).await?;
    } else {
        from_visibilities.0.remove(&file_name);
        from_visibilities.save(from_buf.parent().unwrap()).await?;
        let mut new_visibilities = Visibilities::read_dir(to_buf.parent().unwrap()).await?;
        new_visibilities.0.insert(file_name, from_visibility);
        new_visibilities.save(to_buf.parent().unwrap()).await?;
    }

    Ok(V1Response::Moved)
}
