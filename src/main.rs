//! # MongoDB generic repository in Rust
//!
//! Source code example of how to define a generic repository for a MongoDB database working
//! with different entity structures.
//! 
//! run command "cargo test" to run all database tests
//! 
//! run command "cargo doc --open" to open the documentation in a browser

pub mod database;
pub mod errors;
pub mod tests;

use mongodb::bson::doc;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mdb = database::repository::MongoDB::new().await;

    // get all database collection names
    let coll_list = mdb.db.list_collection_names(doc! {}).await?;
    println!("The list of MongoDB collections: {:?}", coll_list);

    Ok(())
}