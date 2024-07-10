use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

pub trait DbEntity: DeserializeOwned + Sync + Send + Unpin {
    fn collection_name() -> String;
}


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


impl DbEntity for Note {
    fn collection_name() -> String {
        String::from("Notes")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub content: String
}