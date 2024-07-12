#![allow(unused_imports)]
use mongodb::bson::{doc, oid::ObjectId};
use more_asserts::assert_gt;
use std::str::FromStr;

use mongodb_repo::database::entity_user::User;
use mongodb_repo::database::repository::MongoDB;

#[tokio::test]
async fn create_database_document() {
    let mdb = MongoDB::new().await;
    let new_user_id = ObjectId::new();
    let new_user = User::example(&new_user_id);

    let result = mdb.create_document(&new_user).await;

    assert_eq!(new_user_id, result.unwrap());
}

#[tokio::test]
async fn create_and_get_document_by_id() {
    let mdb = MongoDB::new().await;
    let user_id = "65b47748cd37932780900120".to_string();
    let new_user_id = ObjectId::from_str(&user_id).unwrap();
    let new_user = User::example2(&new_user_id);

    let create_result = mdb.create_document(&new_user).await;
    let get_id_result = mdb.get_by_id::<User>(&user_id).await;

    assert_eq!((new_user_id, User::example2(&new_user_id)), (create_result.unwrap(), get_id_result.unwrap()));

    mdb.delete_document::<User>(&new_user_id).await;
}

#[tokio::test]
async fn create_and_update_database_document() {
    let mdb = MongoDB::new().await;
    let new_user_id = ObjectId::new();
    let mut new_user = User::example2(&new_user_id);

    let create_result = mdb.create_document(&new_user).await;

    new_user.username = "Maria".to_string();
    new_user.age = 54;

    let update_result = mdb.update_document::<User>(&new_user_id, &new_user).await;

    assert_eq!((new_user_id, 1), (create_result.unwrap(), update_result.unwrap().modified_count));
}

#[tokio::test]
async fn create_and_delete_database_document() {
    let mdb = MongoDB::new().await;
    let new_user_id = ObjectId::new();
    let new_user = User::example(&new_user_id);

    let create_result = mdb.create_document(&new_user).await;
    let delete_result = mdb.delete_document::<User>(&new_user_id).await;

    assert_eq!((new_user_id, 1), (create_result.unwrap(), delete_result.unwrap().deleted_count));
}

#[tokio::test]
async fn create_and_get_all_database_documents() {
    let mdb = MongoDB::new().await;
    let new_user_id = ObjectId::new();
    let new_user = User::example2(&new_user_id);
    
    mdb.create_document(&new_user).await;
    
    let result_without_filter = mdb.get_all::<User>(None).await;
    let result_with_filter = mdb.get_all::<User>(Some(doc! { "is_male": true })).await;

    assert_gt!((result_without_filter.len(), result_with_filter.len()), (0 as usize, 0 as usize));
}