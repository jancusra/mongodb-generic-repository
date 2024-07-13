pub mod tests;

use mongodb::bson::doc;
use std::error::Error;
use tokio;

use mongodb_repo::database::repository::MongoDB;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mdb = MongoDB::new().await;

    // get all database collection names
    let coll_list = mdb.db.list_collection_names(doc! {}).await?;
    println!("The list of collections for a database {:?}: {:?}", mdb.db.name(), coll_list);

    Ok(())
}