use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use std::str::FromStr;

use crate::database::db_entity::DbEntity;

impl DbEntity for User {
    fn collection_name() -> String {
        String::from("Users")
    }
}

/// Database entity for storing users
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub age: u32,
    pub is_male: bool
}

/// Example of user entities (used for testing)
impl User {
    pub fn example_str_id(id: &str) -> Self {
        User::example(&ObjectId::from_str(id).unwrap())
    }
    
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