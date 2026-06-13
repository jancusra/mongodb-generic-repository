# MongoDB generic repository in Rust

A small, generic repository layer on top of the official [`mongodb`](https://crates.io/crates/mongodb)
driver. Implement a single trait on your entity and you get async CRUD operations
(Create / Read / Update / Delete / Get all) for free — no boilerplate repository per entity.

## Usage

### 1. Define an entity and implement `DbEntity`

```rust
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use mongodb_repo::database::db_entity::DbEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub age: u32,
}

impl DbEntity for User {
    fn collection_name() -> &'static str {
        "Users"
    }
}
```

### 2. Connect and run CRUD operations

```rust
use mongodb::bson::{doc, oid::ObjectId};
use mongodb_repo::database::repository::MongoDB;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    // Connect to mongodb://localhost:27017 and use the "my_app" database.
    let mdb = MongoDB::new("my_app").await?;

    let user_id = ObjectId::new();
    let mut user = User {
        id: Some(user_id),
        username: "Jan".to_string(),
        age: 25,
    };

    // CREATE
    mdb.create_document(&user).await?;

    // READ by id
    let found = mdb.get_by_id::<User>(&user_id).await?;
    println!("{found:?}");

    // UPDATE
    user.age = 26;
    let update_result = mdb.update_document::<User>(&user_id, &user).await?;
    println!("modified: {}", update_result.modified_count);

    // GET ALL
    let all_users = mdb.get_all::<User>().await?;
    // ... or with an optional filter / limit / skip / sort
    let adults = mdb
        .get_all_with_options::<User>(Some(doc! { "age": { "$gte": 18 } }), None, None, None)
        .await?;
    // first 10 users, sorted by age descending
    let page = mdb
        .get_all_with_options::<User>(None, Some(10), None, Some(doc! { "age": -1 }))
        .await?;
    println!("total: {}, adults: {}, page: {}", all_users.len(), adults.len(), page.len());

    // DELETE
    let delete_result = mdb.delete_document::<User>(&user_id).await?;
    println!("deleted: {}", delete_result.deleted_count);

    Ok(())
}
```

### Custom connection options

Use `new` for a quick localhost connection, or `new_with_options` to pass your own
[`ClientOptions`] (custom host, credentials, pool size, TLS, …):

```rust
use mongodb::options::ClientOptions;
use mongodb_repo::database::repository::MongoDB;

# async fn run() -> Result<(), mongodb::error::Error> {
let mut client_options = ClientOptions::parse("mongodb://user:pass@localhost:27017").await?;
client_options.app_name = Some("my_app".to_string());

let mdb = MongoDB::new_with_options(client_options, "my_app").await?;
# Ok(())
# }
```

## Installation

```toml
[dependencies]
mongodb-repo = "0.8"
mongodb = "3"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

Requires a running MongoDB instance (the examples and tests assume `mongodb://localhost:27017`)
and `tokio` as the async runtime in your application.

## Features

* **One generic repository for every entity** — implement the [`DbEntity`] trait
  (a single `collection_name()` method) and all operations work for that type. The
  required bounds (`Serialize`, `DeserializeOwned`, `Send`, `Sync`, `Unpin`) live on the
  trait itself, so your method calls stay clean (`mdb.get_by_id::<User>(&id)`).
* **Safe updates** — `update_document` builds a `$set` update and automatically removes
  the immutable `_id` field, so updating a document never fails on MongoDB's immutable-`_id`
  rule and `_id` is used purely as the filter.
* **Hermetic integration tests** — each integration test runs against its own randomly
  named database and drops it on teardown, so tests are isolated, can run in parallel,
  and never leave residual data behind.
* **Async** — built on `tokio` and the async MongoDB driver.

## API overview

| Method | Description |
| --- | --- |
| `MongoDB::new(db_name)` | Connect to `mongodb://localhost:27017` and select a database |
| `MongoDB::new_with_options(opts, db_name)` | Connect using custom `ClientOptions` |
| `get_by_id::<T>(&id)` | Fetch one document by its `ObjectId` (`Option<T>`) |
| `create_document(&entity)` | Insert one document |
| `update_document::<T>(&id, &entity)` | Replace a document's fields via `$set` |
| `delete_document::<T>(&id)` | Delete one document by its `ObjectId` |
| `get_all::<T>()` | Fetch all documents |
| `get_all_with_options::<T>(filter, limit, skip, sort)` | Fetch documents with optional filter, pagination and sort |

> **Note:** documents are keyed by `ObjectId` — your entity's `_id` field must be an
> `Option<ObjectId>`.

## Running the tests

The integration tests need a running MongoDB instance:

```sh
cargo test            # runs integration tests + documentation tests
cargo doc --open      # open the generated API documentation
```

## License

Licensed under either of MIT or Apache-2.0 at your option.

[`DbEntity`]: https://docs.rs/mongodb-repo/latest/mongodb_repo/database/db_entity/trait.DbEntity.html
[`ClientOptions`]: https://docs.rs/mongodb/latest/mongodb/options/struct.ClientOptions.html
