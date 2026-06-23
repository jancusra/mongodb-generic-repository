use serde::{de::DeserializeOwned, Serialize};

/// Common database entity trait for defining the entity collection name
pub trait DbEntity: Serialize + DeserializeOwned + Send + Sync + Unpin {
    fn collection_name() -> &'static str;
}
