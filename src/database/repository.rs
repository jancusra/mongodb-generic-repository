use futures::TryStreamExt;
use mongodb::{bson, Client, Database};
use mongodb::bson::{doc, Document, oid::ObjectId};
use mongodb::options::ClientOptions;
use mongodb::results::{DeleteResult, UpdateResult};
use serde::{de::DeserializeOwned, Serialize};
use std::str::FromStr;

use crate::database::db_entity::DbEntity;
use crate::errors::{err, MyError};

/// Custom MongoDB implementation
pub struct MongoDB {
    pub db: Database
}

impl MongoDB {
    /// Create new Mongo database connection
    /// 
    /// # Example
    /// 
    /// ```
    /// use mongodb_repo::database::repository::MongoDB;
    /// 
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new().await;
    /// assert_eq!("test_repo", mdb.db.name());
    /// # })
    /// ```
    pub async fn new() -> Self {
        match ClientOptions::parse("mongodb://localhost:27017/").await {
            Ok(client_options) => match Client::with_options(client_options) {
                Ok(client) => return Self {
                    db: client.database("test_repo")
                },
                Err(e) => panic!("{}", e.to_string())
            },
            Err(e) => panic!("{}", e.to_string())
        }
    }

    /// Get database entity by ID
    /// 
    /// # Example
    ///
    /// ```
    /// use mongodb_repo::database::{entity_user::User, repository::MongoDB};
    /// 
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new().await;
    /// let user_id = "65b47748cd37932780900120".to_string();
    /// let user_result = mdb.get_by_id::<User>(&user_id).await;
    /// 
    /// assert_eq!(User::example_str_id(&user_id), user_result.unwrap());
    /// # })
    /// ```
    pub async fn get_by_id<T: DbEntity>(&self, id: &str)
        -> Option<T> where T: DbEntity + DeserializeOwned + Unpin + Send + Sync
    {
        match ObjectId::from_str(id).map_err(err!()) {
            Ok(object_id) =>
                match self.db.collection::<T>(&T::collection_name())
                    .find_one(
                        doc! {
                            "_id": Some(object_id)
                        },
                        None
                    ).await.map_err(err!()) {
                        Ok(entity) => entity,
                        Err(_) => None
                    }
            Err(_) => None
        }
    }

    /// Create database document
    /// 
    /// # Example
    ///
    /// ```
    /// use mongodb::bson::oid::ObjectId;
    /// use mongodb_repo::database::{entity_user::User, repository::MongoDB};
    /// 
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new().await;
    /// let new_user_id = ObjectId::new();
    /// let new_user = User::example2(&new_user_id);
    /// let result = mdb.create_document(&new_user).await;
    ///
    /// assert_eq!(new_user_id, result.unwrap());
    /// # })
    /// ```
    pub async fn create_document<T: DbEntity>(&self, entity: &T) 
        -> Option<ObjectId> where T: DbEntity + Serialize + Unpin + Send + Sync
    {
        match self.db.collection::<T>(&T::collection_name())
            .insert_one(
                entity, 
                None
            ).await.map_err(err!()) {
                Ok(result) => result.inserted_id.as_object_id(),
                Err(_) => None
            }
    }

    /// Update database document by entity ID
    /// 
    /// # Example
    ///
    /// ```
    /// use mongodb::bson::oid::ObjectId;
    /// use mongodb_repo::database::{entity_user::User, repository::MongoDB};
    /// 
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new().await;
    /// let new_user_id = ObjectId::new();
    /// let mut new_user = User::example2(&new_user_id);
    /// let create_result = mdb.create_document(&new_user).await;
    /// 
    /// new_user.username = "Nela - updated by test doc".to_string();
    /// new_user.age = 42;
    /// let update_result = mdb.update_document::<User>(&new_user_id, &new_user).await;
    /// 
    /// assert_eq!((new_user_id, 1), (create_result.unwrap(), update_result.unwrap().modified_count));
    /// # })
    /// ```
    pub async fn update_document<T: DbEntity>(&self, id: &ObjectId, entity: &T) 
        -> Option<UpdateResult> where T: DbEntity + Serialize + Unpin + Send + Sync
    {
        match bson::to_bson(entity).map_err(err!()) {
            Ok(serialized_data) => match serialized_data.as_document() {
                Some(document) => match self.db.collection::<T>(&T::collection_name())
                    .update_one(
                        doc! {
                            "_id": Some(id)
                        },
                        doc! {
                            "$set": document
                        }, 
                        None
                    ).await.map_err(err!()) {
                        Ok(result) => Some(result),
                        Err(_) => None
                    }
                None => None
            },
            Err(_) => None
        }
    }

    pub async fn delete_document<T: DbEntity>(&self, id: &ObjectId) 
        -> Option<DeleteResult> where T: DbEntity + Serialize + Unpin + Send + Sync
    {
        match self.db.collection::<T>(&T::collection_name())
            .delete_one(
                doc! {
                    "_id": Some(id)
                },
                None
            ).await.map_err(err!()) {
                Ok(result) => Some(result),
                Err(_) => None
            }
    }

    pub async fn get_all<T: DbEntity>(&self, filter: Option<Document>)
        -> Vec<T> where T: DbEntity + DeserializeOwned + Unpin + Send + Sync 
    {
        let mut documents: Vec<T> = Vec::new();
        let mut cursor = match self.db.collection::<T>(&T::collection_name())
            .find(
                filter, 
                None
            ).await.map_err(err!()) {
                Ok(cursor) => cursor,
                Err(_) => return vec![]
            };

        while let Ok(Some(doc)) = cursor.try_next().await {
            documents.push(doc);
        }

        documents
    }
}