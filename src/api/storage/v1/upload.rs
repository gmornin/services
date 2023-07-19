use actix_multipart::Multipart;
use actix_web::{web::Path, *};
use std::{error::Error, path::PathBuf};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use crate::{functions::*, structs::*, *};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

#[post("/upload/{token}/{path:.*}")]
pub async fn upload(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    from_res(upload_task(payload, path, req).await)
}

async fn upload_task(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Result<V1Response, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let user_path = PathBuf::from(path.trim_start_matches('/'));

    if !editable(&user_path) || has_dotdot(&user_path) {
        println!("{user_path:?} {}", editable(&user_path));
        return Err(V1Error::PermissionDenied.into());
    }

    let path_buf = get_user_dir(account.id, None).join(user_path);

    if fs::try_exists(&path_buf).await? {
        return Err(V1Error::PathOccupied.into());
    }

    if account
        .exceeds_limit(
            STORAGE_LIMITS.get().unwrap(),
            Some(
                req.headers()
                    .get("content-length")
                    .unwrap()
                    .to_str()?
                    .parse::<u64>()?,
            ),
            None,
        )
        .await?
    {
        return Err(V1Error::FileTooLarge.into());
    }

    let data = bytes_from_multipart(payload).await?;

    if !data.is_empty() {
        let expected = MIME_DB
            .get()
            .unwrap()
            .get_mime_types_from_file_name(path_buf.file_name().unwrap().to_str().unwrap());
        let expected_collapsed = expected
            .iter()
            .map(|mime| mime_collapse(mime.essence_str()))
            .collect::<Vec<_>>();
        match MIME_DB.get().unwrap().get_mime_type_for_data(&data) {
            Some((mime, _))
                if !expected.is_empty() && !expected_collapsed.contains(&mime.essence_str()) =>
            {
                return Err(V1Error::FileTypeMismatch {
                    expected: expected[0].to_string(),
                    got: mime.to_string(),
                }
                .into());
            }
            _ => {}
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path_buf)
        .await?;

    file.write_all(&data).await?;

    Ok(V1Response::FileItemCreated)
}
