//! Integration tests for database entity User

#![allow(unused_imports)]
use mongodb::bson::{doc, oid::ObjectId};
use more_asserts::assert_gt;
use std::str::FromStr;

use crate::database::repository::MongoDB;
use crate::test::entity::user::User;

#[tokio::test]
async fn get_or_create_document_by_id() {
    let mdb = MongoDB::new().await.unwrap();
    let new_user_id = ObjectId::from_str("65b47748cd37932780900120").unwrap();
    let mut user_result = mdb.get_by_id::<User>(&new_user_id).await.unwrap();

    if let Some(user) = user_result {
        assert_eq!(User::example(&new_user_id), user);
    } else {
        let new_user = User::example(&new_user_id);
        let create_result = mdb.create_document(&new_user).await.unwrap();
        user_result = mdb.get_by_id::<User>(&new_user_id).await.unwrap();

        assert_eq!(
            (new_user_id, Some(User::example(&new_user_id))),
            (
                create_result.inserted_id.as_object_id().unwrap(),
                user_result
            )
        );
    }
}

#[tokio::test]
async fn create_database_document() {
    let mdb = MongoDB::new().await.unwrap();
    let new_user_id = ObjectId::new();
    let new_user = User::example2(&new_user_id);

    let result = mdb.create_document(&new_user).await.unwrap();

    assert_eq!(new_user_id, result.inserted_id.as_object_id().unwrap());
}

#[tokio::test]
async fn create_and_update_database_document() {
    let mdb = MongoDB::new().await.unwrap();
    let new_user_id = ObjectId::new();
    let mut new_user = User::example2(&new_user_id);

    let create_result = mdb.create_document(&new_user).await.unwrap();

    new_user.username = "Maria".to_string();
    new_user.age = 54;

    let update_result = mdb
        .update_document::<User>(&new_user_id, &new_user)
        .await
        .unwrap();

    assert_eq!(
        (new_user_id, 1),
        (
            create_result.inserted_id.as_object_id().unwrap(),
            update_result.modified_count
        )
    );
}

#[tokio::test]
async fn create_and_delete_database_document() {
    let mdb = MongoDB::new().await.unwrap();
    let new_user_id = ObjectId::new();
    let new_user = User::example(&new_user_id);

    let create_result = mdb.create_document(&new_user).await.unwrap();
    let delete_result = mdb.delete_document::<User>(&new_user_id).await.unwrap();

    assert_eq!(
        (new_user_id, 1),
        (
            create_result.inserted_id.as_object_id().unwrap(),
            delete_result.deleted_count
        )
    );
}

#[tokio::test]
async fn create_and_get_all_database_documents() {
    let mdb = MongoDB::new().await.unwrap();
    let new_user_id = ObjectId::new();
    let new_user = User::example(&new_user_id);

    mdb.create_document(&new_user).await.unwrap();

    let result_without_filter = mdb.get_all::<User>(None).await.unwrap();
    let result_with_filter = mdb
        .get_all::<User>(Some(doc! { "is_male": true }))
        .await
        .unwrap();

    assert_gt!(
        (result_without_filter.len(), result_with_filter.len()),
        (0 as usize, 0 as usize)
    );
}
