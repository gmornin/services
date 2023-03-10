use async_trait::async_trait;
use mongodb::{bson::doc, Collection};
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait CollectionItem
where
    Self: Sized + Clone + Send + Serialize + DeserializeOwned,
{
    /// update if exist, create or else
    async fn save_generic(
        &self,
        collection: &Collection<Self>,
    ) -> Result<(), mongodb::error::Error> {
        if collection
            .find_one_and_replace(doc! {"_id": self.id()}, self, None)
            .await?
            .is_none()
        {
            collection.insert_one(self, None).await?;
        };
        Ok(())
    }

    /// replaces item
    async fn save_replace(
        &self,
        collection: &Collection<Self>,
    ) -> Result<(), mongodb::error::Error> {
        collection
            .find_one_and_replace(doc! {"_id": self.id()}, self, None)
            .await?;
        Ok(())
    }

    /// creates item
    async fn save_create(
        &self,
        collection: &Collection<Self>,
    ) -> Result<(), mongodb::error::Error> {
        collection.insert_one(self, None).await?;
        Ok(())
    }

    /// deletes self
    async fn delete(&self, collection: &Collection<Self>) -> Result<(), mongodb::error::Error> {
        collection
            .find_one_and_delete(doc! {"_id": self.id()}, None)
            .await?;
        Ok(())
    }

    fn id(&self) -> &str;
}
