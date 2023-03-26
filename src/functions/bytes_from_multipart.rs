use actix_multipart::{Multipart, MultipartError};
use futures_util::TryStreamExt;

pub async fn bytes_from_multipart(mut payload: Multipart) -> Result<Vec<u8>, MultipartError> {
    let mut file_data: Vec<u8> = Vec::new();
    // let mut layout: Option<String> = Some(String::from("simple"));
    #[allow(clippy::single_match)]
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().unwrap();
        match field_name {
            "file" => {
                while let Some(chunk) = field.try_next().await? {
                    file_data.extend_from_slice(&chunk);
                }
            }
            // "layout" => {
            //     let bytes = field.try_next().await?;
            //     layout = String::from_utf8(bytes.unwrap().to_vec()).ok();
            // }
            _ => {}
        }
    }

    Ok(file_data)
}
