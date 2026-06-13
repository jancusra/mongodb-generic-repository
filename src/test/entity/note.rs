use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::database::db_entity::DbEntity;

impl DbEntity for Note {
    fn collection_name() -> &'static str {
        "Notes"
    }
}

/// Database entity for storing notes
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Note {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub content: String,
}

/// Examples of note entities (used for testing).
///
/// The titles are intentionally chosen so their ascending alphabetical order
/// is "Alps hike", "Barcelona trip", "Cycling trip", "Danube cruise",
/// "Everest base camp" — which lets the sort/skip/limit tests assert a
/// deterministic order.
impl Note {
    pub fn example(id: &ObjectId) -> Self {
        Self {
            id: Some(*id),
            title: "Cycling trip".to_string(),
            content: "14 day trip from Venezia to Brno".to_string(),
        }
    }

    pub fn example2(id: &ObjectId) -> Self {
        Self {
            id: Some(*id),
            title: "Barcelona trip".to_string(),
            content: "Cycling trip around Barcelona".to_string(),
        }
    }

    pub fn example3(id: &ObjectId) -> Self {
        Self {
            id: Some(*id),
            title: "Alps hike".to_string(),
            content: "Weekend hike in the Austrian Alps".to_string(),
        }
    }

    pub fn example4(id: &ObjectId) -> Self {
        Self {
            id: Some(*id),
            title: "Danube cruise".to_string(),
            content: "Boat trip from Bratislava to Budapest".to_string(),
        }
    }

    pub fn example5(id: &ObjectId) -> Self {
        Self {
            id: Some(*id),
            title: "Everest base camp".to_string(),
            content: "Two week trek to the Everest base camp".to_string(),
        }
    }
}
