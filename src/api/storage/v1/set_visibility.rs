use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use crate::{functions::*, structs::*};

use goodmorning_bindings::services::v1::{V1Error, V1PathVisibility, V1Response};

#[post("/set-visibility")]
pub async fn set_visibility(post: Json<V1PathVisibility>) -> HttpResponse {
    from_res(set_visibility_task(post).await)
}

async fn set_visibility_task(post: Json<V1PathVisibility>) -> Result<V1Response, Box<dyn Error>> {
    let mut account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let mut user_path = PathBuf::from(post.path.trim_start_matches('/'));

    if let [_, "Shared", user, ..] = user_path
        .iter()
        .map(|s| s.to_str().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
    {
        account = if let Some(account) = Account::find_by_username(user.to_string()).await? {
            account.v1_restrict_verified()?
        } else {
            return Err(V1Error::FileNotFound.into());
        };
        user_path = [user_path.iter().next().unwrap()]
            .into_iter()
            .chain(user_path.iter().skip(3))
            .collect();
    }

    if !editable(&user_path, &account.services) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

    if !fs::try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    let file_name = path_buf.file_name().unwrap().to_str().unwrap();

    let mut visibilities = Visibilities::read_dir(path_buf.parent().unwrap()).await?;
    match visibilities.0.get(file_name) {
        Some(visibility) if visibility == &post.visibility.into() => {
            return Ok(V1Response::NothingChanged)
        }
        _ => {
            let _ = visibilities
                .0
                .insert(file_name.to_string(), post.visibility.into());
        }
    }
    visibilities.save(path_buf.parent().unwrap()).await?;

    Ok(V1Response::VisibilityChanged)
}
