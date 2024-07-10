use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::database::db_entity::DbEntity;

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