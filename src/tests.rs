#![allow(unused_imports)]
use mongodb::bson::oid::ObjectId;
use std::str::FromStr;

use crate::entities::User;
use crate::repository::MongoDB;

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