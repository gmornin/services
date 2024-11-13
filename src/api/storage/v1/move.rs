use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use goodmorning_bindings::services::v1::{V1Error, V1Response, V1SelfFromTo};

use crate::{functions::*, structs::*, FILE_CHECK};

#[post("/move")]
pub async fn r#move(post: Json<V1SelfFromTo>) -> HttpResponse {
    from_res(move_task(post).await)
}

async fn move_task(post: Json<V1SelfFromTo>) -> Result<V1Response, Box<dyn Error>> {
    let mut account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let mut user_tobuf = PathBuf::from(post.to.trim_start_matches('/'));
    let mut user_frombuf = PathBuf::from(post.from.trim_start_matches('/'));

    match (
        user_tobuf
            .iter()
            .map(|s| s.to_str().unwrap())
            .collect::<Vec<_>>()
            .as_slice(),
        user_frombuf
            .iter()
            .map(|s| s.to_str().unwrap())
            .collect::<Vec<_>>()
            .as_slice(),
    ) {
        ([_, "Shared", u1, ..], [_, "Shared", u2, ..]) if u1 != u2 => {
            return Err(V1Error::PermissionDenied.into())
        }
        ([_, "Shared", _, ..], [_, "Shared"]) => return Err(V1Error::PermissionDenied.into()),
        ([_, "Shared"], _) => return Err(V1Error::PermissionDenied.into()),
        ([_, "Shared", _, ..], [_, dir]) if *dir != "Shared" => {
            return Err(V1Error::PermissionDenied.into())
        }
        ([_, "Shared", _, ..], [_]) => return Err(V1Error::PermissionDenied.into()),
        ([_, "Shared", user, ..], _) => {
            account = if let Some(account) = Account::find_by_username(user.to_string()).await? {
                account.v1_restrict_verified()?
            } else {
                return Err(V1Error::FileNotFound.into());
            };
            user_tobuf = [user_tobuf.iter().next().unwrap()]
                .into_iter()
                .chain(user_tobuf.iter().skip(3))
                .collect();
            user_frombuf = [user_frombuf.iter().next().unwrap()]
                .into_iter()
                .chain(user_frombuf.iter().skip(3))
                .collect();
        }
        _ => {}
    }

    if !editable(&user_tobuf, &account.services)
        || !editable(&user_frombuf, &account.services)
        || has_dotdot(&user_tobuf)
        || has_dotdot(&user_frombuf)
    {
        return Err(V1Error::PermissionDenied.into());
    }

    let to_buf = get_user_dir(account.id, None).join(user_tobuf);
    let from_buf = get_user_dir(account.id, None).join(user_frombuf);

    if to_buf == from_buf {
        return Ok(V1Response::Moved);
    }

    if fs::try_exists(&to_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    if !fs::try_exists(&from_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if !fs::try_exists(&to_buf.parent().unwrap()).await? {
        return Err(V1Error::FileNotFound.into());
    }

    if from_buf.extension() != to_buf.extension() && FileCheckType::None != *FILE_CHECK.get().unwrap() {
        return Err(V1Error::ExtensionMismatch.into());
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
