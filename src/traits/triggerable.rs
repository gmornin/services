use async_trait::async_trait;
use dyn_clone::DynClone;
use mongodb::Database;
use std::error::Error;

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait Triggerable
where
    Self: Send + DynClone + Sync,
{
    async fn init(&self, _db: &Database) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    async fn trigger(&self, _db: &Database) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

dyn_clone::clone_trait_object!(Triggerable);
