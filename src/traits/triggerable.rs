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
    async fn init(&self, _db: &Database, _id: &str, _expire: u64) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    async fn trigger(&self, _db: &Database, _id: &str, _expire: u64) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    async fn revoke(&self, _db: &Database, _id: &str, _expire: u64) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

dyn_clone::clone_trait_object!(Triggerable);
