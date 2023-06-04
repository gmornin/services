use std::error::Error;

use actix_web::{
    get,
    web::{Json, Path},
};

use crate::{functions::*, structs::*, traits::CollectionItem, *};

use goodmorning_bindings::{
    services::v1::{V1Error, V1Response},
    traits::ResTrait,
};

#[get("/revoke/{id}")]
async fn revoke(id: Path<String>) -> Json<V1Response> {
    Json(V1Response::from_res(revoke_task(&id.into_inner()).await))
}

async fn revoke_task(id: &str) -> Result<V1Response, Box<dyn Error>> {
    let triggers = get_triggers(DATABASE.get().unwrap());
    let trigger = match Trigger::find_by_id(id, &triggers).await? {
        Some(trigger) => trigger,
        None => return Err(V1Error::TriggerNotFound.into()),
    };

    if trigger.is_invalid() {
        trigger.revoke(DATABASE.get().unwrap()).await?;
        trigger.delete(&triggers).await?;
        return Err(V1Error::TriggerNotFound.into());
    }

    trigger.revoke(DATABASE.get().unwrap()).await?;

    Ok(V1Response::Revoked)
}
