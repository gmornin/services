use std::error::Error;

use crate::{functions::*, structs::*, *};
use actix_multipart::Multipart;
use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use goodmorning_bindings::services::v1::{V1Error, V1ProfileOnly, V1Response};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

#[post("/set-pfp")]
async fn set_pfp(post: Json<V1ProfileOnly>, payload: Multipart, req: HttpRequest) -> HttpResponse {
    from_res(set_pfp_task(post, payload, req).await)
}

async fn set_pfp_task(
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

    let path = get_usersys_dir(account.id, Some("tex")).join("pfp.png");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .await?;

    let data = bytes_from_multipart(payload).await?;

    match MIME_DB.get().unwrap().get_mime_type_for_data(&data) {
        Some((mime, _)) if mime != mime::IMAGE_PNG => {
            return Err(V1Error::FileTypeMismatch {
                expected: mime::IMAGE_PNG.to_string(),
                got: mime.to_string(),
            }
            .into());
        }
        Some(_) => file.write_all(&data).await?,
        _ => {
            return Err(V1Error::FileTypeMismatch {
                expected: mime::IMAGE_PNG.to_string(),
                got: String::from("unknown"),
            }
            .into());
        }
    }

    Ok(V1Response::ProfileUpdated)
}
