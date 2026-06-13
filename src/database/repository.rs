use crate::database::db_entity::DbEntity;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::error::Error;
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{bson, Client, Database};

/// Custom MongoDB implementation
#[derive(Clone, Debug)]
pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    /// Create new Mongo database connection
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mongodb_repo::database::repository::MongoDB;
    ///
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new("test_repo").await.unwrap();
    /// assert_eq!("test_repo", mdb.db.name());
    /// # })
    /// ```
    pub async fn new(database_name: &str) -> Result<Self, Error> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017/").await?;

        Self::new_with_options(client_options, database_name).await
    }

    /// Create new Mongo database connection from custom client options
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mongodb::options::ClientOptions;
    /// use mongodb_repo::database::repository::MongoDB;
    ///
    /// # tokio_test::block_on(async {
    /// let client_options = ClientOptions::parse("mongodb://localhost:27017/").await.unwrap();
    /// let mdb = MongoDB::new_with_options(client_options, "test_repo").await.unwrap();
    /// assert_eq!("test_repo", mdb.db.name());
    /// # })
    /// ```
    pub async fn new_with_options(
        client_options: ClientOptions,
        database_name: &str,
    ) -> Result<Self, Error> {
        let client = Client::with_options(client_options)?;

        Ok(Self {
            db: client.database(database_name),
        })
    }

    /// Get database entity by ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mongodb::bson::oid::ObjectId;
    /// use serde::{Serialize, Deserialize};
    /// use mongodb_repo::database::{db_entity::DbEntity, repository::MongoDB};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ///     id: Option<ObjectId>,
    ///     username: String,
    /// }
    ///
    /// impl DbEntity for User {
    ///     fn collection_name() -> &'static str { "Users" }
    /// }
    ///
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new("test_repo").await.unwrap();
    /// let user_id = ObjectId::new();
    /// let user_result = mdb.get_by_id::<User>(&user_id).await.unwrap();
    ///
    /// if let Some(user) = user_result {
    ///     assert_eq!("Jan", user.username);
    /// }
    /// # })
    /// ```
    pub async fn get_by_id<T: DbEntity>(&self, id: &ObjectId) -> Result<Option<T>, Error> {
        self.db
            .collection::<T>(T::collection_name())
            .find_one(doc! { "_id": *id })
            .await
    }

    /// Create database document
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mongodb::bson::oid::ObjectId;
    /// use serde::{Serialize, Deserialize};
    /// use mongodb_repo::database::{db_entity::DbEntity, repository::MongoDB};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ///     id: Option<ObjectId>,
    ///     username: String,
    /// }
    ///
    /// impl DbEntity for User {
    ///     fn collection_name() -> &'static str { "Users" }
    /// }
    ///
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new("test_repo").await.unwrap();
    /// let new_user_id = ObjectId::new();
    /// let new_user = User { id: Some(new_user_id), username: "Tereza".to_string() };
    /// let result = mdb.create_document(&new_user).await.unwrap();
    ///
    /// assert_eq!(new_user_id, result.inserted_id.as_object_id().unwrap());
    /// # })
    /// ```
    pub async fn create_document<T: DbEntity>(&self, entity: &T) -> Result<InsertOneResult, Error> {
        self.db
            .collection::<T>(T::collection_name())
            .insert_one(entity)
            .await
    }

    /// Update database document by entity ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mongodb::bson::oid::ObjectId;
    /// use serde::{Serialize, Deserialize};
    /// use mongodb_repo::database::{db_entity::DbEntity, repository::MongoDB};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ///     id: Option<ObjectId>,
    ///     username: String,
    /// }
    ///
    /// impl DbEntity for User {
    ///     fn collection_name() -> &'static str { "Users" }
    /// }
    ///
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new("test_repo").await.unwrap();
    /// let new_user_id = ObjectId::new();
    /// let mut new_user = User { id: Some(new_user_id), username: "Tereza".to_string() };
    /// mdb.create_document(&new_user).await.unwrap();
    ///
    /// new_user.username = "Nela - updated by test doc".to_string();
    /// let update_result = mdb.update_document::<User>(&new_user_id, &new_user).await.unwrap();
    ///
    /// assert_eq!(1, update_result.modified_count);
    /// # })
    /// ```
    pub async fn update_document<T: DbEntity>(
        &self,
        id: &ObjectId,
        entity: &T,
    ) -> Result<UpdateResult, Error> {
        let mut document = bson::serialize_to_document(entity)?;

        // The "_id" field is immutable in MongoDB and is already used as the
        // filter, so it must not be part of the "$set" update document.
        document.remove("_id");

        self.db
            .collection::<T>(T::collection_name())
            .update_one(doc! { "_id": *id }, doc! { "$set": document })
            .await
    }

    /// Delete database document by ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mongodb::bson::oid::ObjectId;
    /// use serde::{Serialize, Deserialize};
    /// use mongodb_repo::database::{db_entity::DbEntity, repository::MongoDB};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ///     id: Option<ObjectId>,
    ///     username: String,
    /// }
    ///
    /// impl DbEntity for User {
    ///     fn collection_name() -> &'static str { "Users" }
    /// }
    ///
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new("test_repo").await.unwrap();
    /// let new_user_id = ObjectId::new();
    /// let new_user = User { id: Some(new_user_id), username: "Jan".to_string() };
    ///
    /// mdb.create_document(&new_user).await.unwrap();
    /// let delete_result = mdb.delete_document::<User>(&new_user_id).await.unwrap();
    ///
    /// assert_eq!(1, delete_result.deleted_count);
    /// # })
    /// ```
    pub async fn delete_document<T: DbEntity>(&self, id: &ObjectId) -> Result<DeleteResult, Error> {
        self.db
            .collection::<T>(T::collection_name())
            .delete_one(doc! { "_id": *id })
            .await
    }

    /// Retrieve all documents for a specific entity.
    ///
    /// For filtering, pagination or sorting use
    /// [`get_all_with_options`](Self::get_all_with_options).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mongodb::bson::oid::ObjectId;
    /// use more_asserts::assert_gt;
    /// use serde::{Serialize, Deserialize};
    /// use mongodb_repo::database::{db_entity::DbEntity, repository::MongoDB};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ///     id: Option<ObjectId>,
    ///     username: String,
    /// }
    ///
    /// impl DbEntity for User {
    ///     fn collection_name() -> &'static str { "Users" }
    /// }
    ///
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new("test_repo").await.unwrap();
    /// let new_user = User { id: Some(ObjectId::new()), username: "Jan".to_string() };
    ///
    /// mdb.create_document(&new_user).await.unwrap();
    /// let all_users = mdb.get_all::<User>().await.unwrap();
    ///
    /// assert_gt!(all_users.len(), 0usize);
    /// # })
    /// ```
    pub async fn get_all<T: DbEntity>(&self) -> Result<Vec<T>, Error> {
        self.get_all_with_options::<T>(None, None, None, None).await
    }

    /// Retrieve documents for a specific entity, with an optional filter,
    /// pagination (`limit`, `skip`) and `sort`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mongodb::bson::{doc, oid::ObjectId};
    /// use more_asserts::assert_gt;
    /// use serde::{Serialize, Deserialize};
    /// use mongodb_repo::database::{db_entity::DbEntity, repository::MongoDB};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    ///     id: Option<ObjectId>,
    ///     username: String,
    /// }
    ///
    /// impl DbEntity for User {
    ///     fn collection_name() -> &'static str { "Users" }
    /// }
    ///
    /// # tokio_test::block_on(async {
    /// let mdb = MongoDB::new("test_repo").await.unwrap();
    /// let new_user = User { id: Some(ObjectId::new()), username: "Jan".to_string() };
    ///
    /// mdb.create_document(&new_user).await.unwrap();
    ///
    /// // all matching documents
    /// let result_with_filter = mdb
    ///     .get_all_with_options::<User>(Some(doc! { "username": "Jan" }), None, None, None)
    ///     .await
    ///     .unwrap();
    ///
    /// // first 10 documents, sorted by username ascending
    /// let first_page = mdb
    ///     .get_all_with_options::<User>(None, Some(10), None, Some(doc! { "username": 1 }))
    ///     .await
    ///     .unwrap();
    ///
    /// assert_gt!(result_with_filter.len(), 0usize);
    /// # })
    /// ```
    pub async fn get_all_with_options<T: DbEntity>(
        &self,
        filter: Option<Document>,
        limit: Option<i64>,
        skip: Option<u64>,
        sort: Option<Document>,
    ) -> Result<Vec<T>, Error> {
        let options = FindOptions::builder()
            .limit(limit)
            .skip(skip)
            .sort(sort)
            .build();

        let cursor = self
            .db
            .collection::<T>(T::collection_name())
            .find(filter.unwrap_or_default())
            .with_options(options)
            .await?;

        let documents: Vec<T> = cursor.try_collect().await?;

        Ok(documents)
    }
}
