use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::database::db_entity::DbEntity;

impl DbEntity for User {
    fn collection_name() -> String {
        String::from("Users")
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub age: u32,
    pub is_male: bool
}

impl User {
    pub fn example(id: &ObjectId) -> Self {
        Self {
            id: Some(id.clone()),
            username: "Jan".to_string(),
            age: 25,
            is_male: true
        }
    }

    pub fn example2(id: &ObjectId) -> Self {
        Self {
            id: Some(id.clone()),
            username: "Tereza".to_string(),
            age: 30,
            is_male: false
        }
    }
}