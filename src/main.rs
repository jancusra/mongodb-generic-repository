pub mod database;
pub mod errors;
pub mod tests;

use mongodb::bson::doc;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mdb = database::repository::MongoDB::new().await;

    let coll_list = mdb.db.list_collection_names(doc! {}).await?;
    println!("The list of MongoDB collections: {:?}", coll_list);

    Ok(())
}