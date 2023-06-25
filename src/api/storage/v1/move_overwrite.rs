use actix_web::{web::Json, *};
use std::error::Error;
use tokio::fs;

use goodmorning_bindings::services::v1::{V1Error, V1Response, V1SelfFromTo};

use crate::{functions::*, structs::*, *};

#[post("/move-overwrite/{path:.*}")]
pub async fn r#move(post: Json<V1SelfFromTo>) -> HttpResponse {
    from_res(move_overwrite_task(post).await)
}

async fn move_overwrite_task(post: Json<V1SelfFromTo>) -> Result<V1Response, Box<dyn Error>> {
    let accounts = ACCOUNTS.get().unwrap();
    let account = match Account::find_by_token(&post.token, accounts).await? {
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

    let to_buf = get_user_dir(account.id, None).join(post.to.trim_start_matches('/'));
    let from_buf = get_user_dir(account.id, None).join(post.from.trim_start_matches('/'));

    if !editable(&to_buf) || !editable(&from_buf) || has_dotdot(&to_buf) || has_dotdot(&from_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if !fs::try_exists(&from_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if from_buf.extension() != to_buf.extension() {
        return Err(V1Error::ExtensionMismatch.into());
    }

    if fs::metadata(&to_buf).await?.is_dir() {
        fs::remove_dir_all(&to_buf).await?;
    } else {
        fs::remove_file(&to_buf).await?;
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
        let mut new_visibilities = Visibilities::read_dir(to_buf.parent().unwrap()).await?;
        new_visibilities.0.insert(file_name, from_visibility);
        new_visibilities.save(to_buf.parent().unwrap()).await?;
    }

    Ok(V1Response::Moved)
}
