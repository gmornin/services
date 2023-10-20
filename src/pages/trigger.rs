use std::error::Error;

use actix_files::NamedFile;
use actix_web::{get, web::Path, HttpRequest, HttpResponse};

use crate::{structs::Trigger, traits::CollectionItem, SERVICES_STATIC, TRIGGERS};

#[get("/trigger/{id}")]
pub async fn trigger(id: Path<String>, req: HttpRequest) -> HttpResponse {
    match trigger_task(id, &req).await {
        Ok(res) => res,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn trigger_task(id: Path<String>, req: &HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let trig = match Trigger::find_by_id(id.into_inner(), TRIGGERS.get().unwrap()).await? {
        None => {
            return Ok(NamedFile::open_async(
                SERVICES_STATIC
                    .get()
                    .unwrap()
                    .join("htmls/triggernotfound.html"),
            )
            .await?
            .into_response(req))
        }
        Some(t) => t,
    };

    match trig.peek().and_then(|peek| peek.to_html()) {
        Some(pk) => Ok(pk),
        None => Ok(NamedFile::open_async(
            SERVICES_STATIC.get().unwrap().join("htmls/unpeekable.html"),
        )
        .await?
        .into_response(req)),
    }
}
