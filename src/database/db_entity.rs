use serde::de::DeserializeOwned;

/// Common database entity trait for defining the entity table name
pub trait DbEntity: DeserializeOwned + Sync + Send + Unpin {
    fn collection_name() -> String;
}