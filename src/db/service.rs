use actix_web::error;
use anyhow::anyhow;
use async_trait::async_trait;
use bson::Document;
use bson::oid::ObjectId;
use bson::ordered::OrderedDocument;
use log::*;
use mongodb::Collection;
use mongodb::error::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::utils::common::{ApplicationError, CursorAsVec, struct_to_document};

#[async_trait(? Send)]
pub trait MongodbCrudService<T>
    where
        T: 'static + DeserializeOwned + Serialize,
{
    fn new() -> Self;

    fn table(&self) -> Collection;

    // return inserted id
    async fn db_create_resource(&self, record: T) -> Result<T, ApplicationError> {
        let d: Document = struct_to_document(&record).unwrap();
        let rs = self.table().insert_one(d, None)?;
        let new_id: String = rs
            .inserted_id
            .as_object_id()
            .map(ObjectId::to_hex)
            .unwrap();
        info!("saved resource, id={}", new_id);
        let res = self.table().find_one(
            Some(doc! {"_id" => ObjectId::with_string(&new_id).unwrap()}),
            None,
        ).unwrap();
        match res {
            None => Err(ApplicationError::InternalError {
                source: anyhow!("resource not found after insertion")
            }),
            Some(doc) => Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
        }
    }

    async fn db_read_all_resources(&self) -> Result<Vec<T>, Error> {
        let coll = self.table();
        let cursor = coll.find(None, None);
        let res = cursor.map(|mut x| x.as_vec::<T>()).unwrap();
        info!("Retrieving all address objects, count {}", res.len());
        Ok(res)
    }

    async fn db_read_resource(
        &self,
        id: &str,
    ) -> Result<T, ApplicationError> {
        let coll = self.table();
        let res = coll.find_one(
            Some(doc! {"_id" => ObjectId::with_string(id).unwrap()}),
            None,
        ).unwrap();
        info!("Retrieving address with id: {}", id);
        match res {
            None => Err(ApplicationError::InternalError {
                source: anyhow!("resource not found")
            }),
            Some(doc) => Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
        }
    }

    // return modified count
    async fn update_by_oid(&self, oid: ObjectId, record: &T) -> Result<T, ApplicationError> {
        log::info!("update_by_oid");
        let filter = doc! {"_id": oid};

        let d: Document = struct_to_document(record).unwrap();
        let update = doc! {"$set": d};
        let result = self.table().find_one_and_update(filter, update, None).unwrap();
        match result {
            None => Err(ApplicationError::InternalError {
                source: anyhow!("resource not found while update")
            }),
            Some(doc) => Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
        }
    }

    // return deleted count
    async fn remove_by_oid(&self, id: &str) -> Result<i64, Error> {
        let filter = doc! {"_id":  ObjectId::with_string(id).unwrap()};

        let result = self.table().delete_one(filter, None).unwrap();
        Ok(result.deleted_count)
    }
}
