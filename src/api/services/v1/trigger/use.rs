use std::error::Error;

use actix_web::{get, web::Path, HttpResponse};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

use goodmorning_bindings::services::v1::{V1Error, V1Response};

#[get("/use/{id}")]
async fn r#use(path: Path<String>) -> HttpResponse {
    from_res(use_task(path).await)
}

async fn use_task(id: Path<String>) -> Result<V1Response, Box<dyn Error>> {
    let triggers = get_triggers(DATABASE.get().unwrap());
    let trigger = match Trigger::find_by_id(&id, &triggers).await? {
        Some(trigger) => trigger,
        None => return Err(V1Error::TriggerNotFound.into()),
    };

    if trigger.is_invalid() {
        trigger.revoke(DATABASE.get().unwrap()).await?;
        trigger.delete(&triggers).await?;
        return Err(V1Error::TriggerNotFound.into());
    }

    trigger.trigger(DATABASE.get().unwrap()).await?;

    Ok(V1Response::Triggered)
}
