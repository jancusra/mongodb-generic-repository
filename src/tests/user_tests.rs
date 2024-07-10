#![allow(unused_imports)]
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;

use crate::database::entities::User;
use crate::database::repository::MongoDB;

#[tokio::test]
async fn create_database_document() {
    let mdb = MongoDB::new().await;
    let new_user_id = Some(ObjectId::from_str("65b47748cd37932780900120").unwrap());

    let new_user = User {
        id: new_user_id,
        username: "Jan".to_string(),
        age: 25,
        is_male: true
    };

    let result = mdb.create_document(&new_user).await;

    assert_eq!(new_user_id, result);
}

/*#[tokio::test]
async fn create_another_database_document() {
    let mdb = MongoDB::new().await;
    let new_user_id = Some(ObjectId::from_str("65b47748cd379327809001f5").unwrap());

    let new_user = User {
        id: new_user_id,
        username: "Katarina".to_string(),
        age: 36,
        is_male: false
    };

    let result = mdb.create_document(&new_user).await;

    assert_eq!(new_user_id, result);
}

#[tokio::test]
async fn get_document_by_id() {
    let mdb = MongoDB::new().await;
    let user_id = "65b47748cd37932780900120".to_string();

    let user_to_get = User {
        id: Some(ObjectId::from_str(&user_id).unwrap()),
        username: "Jan".to_string(),
        age: 25,
        is_male: true
    };

    let result = mdb.get_by_id::<User>(&user_id).await.unwrap();

    assert_eq!(user_to_get, result);
}

#[tokio::test]
async fn get_document_by_filter() {
    let mdb = MongoDB::new().await;

    let user_to_get = User {
        id: Some(ObjectId::from_str("65b47748cd379327809001f5").unwrap()),
        username: "Katarina".to_string(),
        age: 36,
        is_male: false
    };

    let result = mdb.get_one_by_filter::<User>(doc! { "username": "Katarina" }).await.unwrap();

    assert_eq!(user_to_get, result);
}*/