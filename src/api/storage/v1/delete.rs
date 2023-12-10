use actix_web::{web::Json, *};
use std::{error::Error, path::PathBuf};
use tokio::fs;

use crate::{functions::*, structs::*, traits::CollectionItem, ACCOUNTS};

use goodmorning_bindings::services::v1::{V1Error, V1PathOnly, V1Response};

#[post("/delete")]
pub async fn delete(post: Json<V1PathOnly>) -> HttpResponse {
    from_res(delete_task(post).await)
}

async fn delete_task(post: Json<V1PathOnly>) -> Result<V1Response, Box<dyn Error>> {
    let mut account = Account::v1_get_by_token(&post.token)
        .await?
        .v1_restrict_verified()?;

    let user_path = PathBuf::from(post.path.trim_start_matches('/'));
    if !editable(&user_path, &account.services) || has_dotdot(&user_path) {
        return Err(V1Error::PermissionDenied.into());
    }
    let path_buf = get_user_dir(account.id, None).join(user_path);

    if !fs::try_exists(&path_buf).await? {
        return Err(V1Error::FileNotFound.into());
    }

    let meta = fs::metadata(&path_buf).await?;
    if meta.is_file() {
        fs::remove_file(&path_buf).await?;
        if let Some(stored) = account.stored.as_mut()
            && meta.len() != 0
        {
            stored.value = stored.value.saturating_sub(meta.len());
            account.save_replace(ACCOUNTS.get().unwrap()).await?;
        }
    } else {
        let size = dir_size(&path_buf).await?;
        fs::remove_dir_all(&path_buf).await?;
        if let Some(stored) = account.stored.as_mut()
            && size != 0
        {
            stored.value = stored.value.saturating_sub(size);
            account.save_replace(ACCOUNTS.get().unwrap()).await?;
        }
    }

    let mut visibilities = Visibilities::read_dir(path_buf.parent().unwrap()).await?;
    let file_name = path_buf
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    if visibilities.0.contains_key(&file_name) {
        visibilities.0.remove(&file_name);
    }
    visibilities.save(path_buf.parent().unwrap()).await?;

    Ok(V1Response::FileItemDeleted)
}
