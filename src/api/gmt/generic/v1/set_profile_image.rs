use std::{error::Error, path::PathBuf};

use crate::{functions::*, structs::*, *};
use actix_multipart::Multipart;
use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1ProfileOnly, V1Response};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

#[post("/set-profile-image")]
async fn set_profile_image(
    post: Json<V1ProfileOnly>,
    payload: Multipart,
    req: HttpRequest,
) -> HttpResponse {
    from_res(set_profile_image_task(post, payload, req).await)
}

async fn set_profile_image_task(
    post: Json<V1ProfileOnly>,
    payload: Multipart,
    req: HttpRequest,
) -> Result<V1Response, Box<dyn Error>> {
    let post = post.into_inner();
    let accounts = get_accounts(DATABASE.get().unwrap());

    let account = match Account::find_by_token(&post.token, &accounts).await? {
        Some(account) => account,
        None => return Err(V1Error::InvalidToken.into()),
    };

    if !account.verified {
        return Ok(V1Response::Error {
            kind: V1Error::NotVerified,
        });
    }

    if *PFP_LIMIT.get().unwrap()
        < req
            .headers()
            .get("content-length")
            .unwrap()
            .to_str()?
            .parse::<u64>()?
    {
        return Err(V1Error::FileTooLarge.into());
    }

    let path = PathBuf::from(USERCONTENT.get().unwrap().as_str())
        .join(account.id.to_string())
        .join("gmt")
        .join(".system")
        .join("profile_image");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .await?;

    let data = bytes_from_multipart(payload).await?;

    match MIME_DB.get().unwrap().get_mime_type_for_data(&data) {
        Some((mime, _)) if mime != mime::IMAGE_PNG && mime != mime::IMAGE_JPEG => {
            return Err(V1Error::FileTypeMismatch {
                expected: mime::IMAGE_PNG.to_string(),
                got: mime.to_string(),
            }
            .into());
        }
        _ => {}
    }

    file.write_all(&data).await?;

    Ok(V1Response::ProfileUpdated)
}
