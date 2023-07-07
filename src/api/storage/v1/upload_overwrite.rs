use actix_multipart::Multipart;
use actix_web::{web::Path, *};
use std::error::Error;
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

use crate::{functions::*, structs::*, *};

#[post("/upload-overwrite/{token}/{path:.*}")]
pub async fn upload_overwrite(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> HttpResponse {
    from_res(upload_overwrite_task(payload, path, req).await)
}

async fn upload_overwrite_task(
    payload: Multipart,
    path: Path<(String, String)>,
    req: HttpRequest,
) -> Result<V1Response, Box<dyn Error>> {
    let (token, path) = path.into_inner();
    let account = Account::v1_get_by_token(&token)
        .await?
        .v1_restrict_verified()?;

    let path_buf = get_user_dir(account.id, None).join(path.trim_start_matches('/'));

    if !editable(&path_buf) || has_dotdot(&path_buf) {
        return Err(V1Error::PermissionDenied.into());
    }

    if account
        .exceeds_limit(
            STORAGE_LIMITS.get().unwrap(),
            Some(
                req.headers()
                    .get("content-length")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
            ),
            Some(match fs::metadata(&path_buf).await {
                Ok(me) => me.len(),
                Err(_) => 0,
            }),
        )
        .await
        .unwrap()
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
                println!("got: {mime} expected: {expected:?}");
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
        .create(true)
        .truncate(true)
        .write(true)
        .open(&path_buf)
        .await?;

    file.write_all(&data).await?;

    Ok(V1Response::FileItemCreated)
}
