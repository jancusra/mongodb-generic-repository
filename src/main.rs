mod entities;
mod errors;
mod repository;

use entities::User;
use mongodb::bson::oid::ObjectId;
use repository::MongoDB;
use std::error::Error;
use std::str::FromStr;
use tokio;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mdb = MongoDB::new().await;

    let entry = User {
        id: Some(ObjectId::from_str("65b47748cd37932780900120").unwrap()),
        username: "Jan".to_string(),
        age: 25,
        is_male: true
    };

    let result = mdb.create_document(&entry).await;
    //let result = mdb.get_by_id::<User>("65b47748cd37932780900120").await;
    println!("{:?}", result);

    Ok(())
}
