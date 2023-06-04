use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use goodmorning_bindings::{
    services::v1::{V1Error, V1FromTo, V1Response},
    traits::ResTrait,
};

use crate::{functions::*, structs::*, *};

#[post("/move/{path:.*}")]
pub async fn r#move(post: Json<V1FromTo>) -> Json<V1Response> {
    Json(V1Response::from_res(
        move_task(&post.from, &post.to, &post.from_userid, &post.token).await,
    ))
}

async fn move_task(
    from: &str,
    to: &str,
    from_id: &str,
    token: &str,
) -> Result<V1Response, Box<dyn Error>> {
    let accounts = get_accounts(DATABASE.get().unwrap());
    let account = match Account::find_by_token(token, &accounts).await? {
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
        .join(&account.id)
        .join(to.trim_start_matches('/'));
    let from_buf = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(from_id)
        .join(from.trim_start_matches('/'));

    if !editable(&to_buf) || !has_dotdot(&to_buf) || !has_dotdot(&from_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if fs::try_exists(&to_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    if from_id == account.id {
        if !fs::try_exists(&from_buf).await? {
            return Err(V1Error::FileNotFound.into());
        }
    } else if !fs::try_exists(&from_buf).await?
        || Visibilities::visibility(&from_buf).await?.visibility == ItemVisibility::Private
    {
        return Err(V1Error::FileNotFound.into());
    }

    fs::rename(&from_buf, &to_buf).await?;

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

    Ok(V1Response::Moved {
        path: format!("/{}/{}", account.id, to),
    })
}
