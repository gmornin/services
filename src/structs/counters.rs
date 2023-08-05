use std::error::Error;

use crate::{functions::get_counters_doc, COUNTERS};
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Counter {
    pub count: i64,
}

impl Counter {
    pub async fn bump_get(id: &str, _db: &Database) -> Result<i64, Box<dyn Error>> {
        let counters = COUNTERS.get().unwrap();
        let filter = doc! {"_id": id};
        let update = doc! {"$inc": {"count": 1}};
        let exists = counters.find_one(filter.clone(), None).await?.is_some();
        Ok(if exists {
            let options = mongodb::options::FindOneAndUpdateOptions::builder()
                .upsert(true)
                .build();
            counters
                .find_one_and_update(filter, update, options)
                .await?
                .unwrap()
                .count
        } else {
            get_counters_doc()
                .insert_one(doc! {"_id": id, "count": 1}, None)
                .await?;
            1
        })
    }
}
