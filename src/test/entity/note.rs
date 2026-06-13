use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::database::db_entity::DbEntity;

impl DbEntity for Note {
    fn collection_name() -> String {
        String::from("Notes")
    }
}

/// Database entity for storing notes
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Note {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub content: String
}

/// Example of note entity (used for testing)
impl Note {
    pub fn example(id: &ObjectId) -> Self {
        Self {
            id: Some(id.clone()),
            title: "Cycling trip".to_string(),
            content: "14 day trip from Venezia to Brno".to_string()
        }
    }
}