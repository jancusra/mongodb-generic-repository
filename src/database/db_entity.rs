use serde::de::DeserializeOwned;

pub trait DbEntity: DeserializeOwned + Sync + Send + Unpin {
    fn collection_name() -> String;
}