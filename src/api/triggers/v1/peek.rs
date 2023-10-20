use std::error::Error;

use actix_web::{get, web::Path, HttpResponse};
use bindings::traits::SerdeAny;

use crate::{functions::*, structs::*, traits::CollectionItem, *};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

#[get("/peek/{id}")]
async fn peek(path: Path<String>) -> HttpResponse {
    from_res(peek_task(path).await)
}

async fn peek_task(id: Path<String>) -> Result<V1Response, Box<dyn Error>> {
    let triggers = TRIGGERS.get().unwrap();
    let trigger = match Trigger::find_by_id(id.into_inner(), triggers).await? {
        Some(trigger) => trigger,
        None => return Err(V1Error::TriggerNotFound.into()),
    };

    let trigger: Box<dyn SerdeAny> = match trigger.peek() {
        Some(trigger) => trigger,
        None => return Err(V1Error::Unpeakable.into()),
    };

    Ok(V1Response::TriggerPeek { value: trigger })
}
