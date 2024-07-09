pub mod entities;
pub mod errors;
pub mod repository;
pub mod tests;

use mongodb::bson::doc;
use std::error::Error;
use tokio;

use repository::MongoDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mdb = MongoDB::new().await;

    let coll_list = mdb.db.list_collection_names(doc! {}).await?;
    println!("The list of MongoDB collections: {:?}", coll_list);

    Ok(())
}
