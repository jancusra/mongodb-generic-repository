//! Shared helpers that keep integration tests hermetic.

use mongodb::bson::oid::ObjectId;

use crate::database::repository::MongoDB;

/// Connect to a freshly named, isolated test database.
///
/// Every test gets its own randomly named database so tests never share
/// state and can run in parallel without interfering with each other.
/// Pair this with [`drop_db`] at the end of the test for teardown.
pub async fn test_db() -> MongoDB {
    let db_name = format!("test_repo_{}", ObjectId::new());

    MongoDB::new(&db_name).await.unwrap()
}

/// Drop the whole test database (teardown).
///
/// Call this *before* the assertions so the database is cleaned up even
/// when an assertion fails and panics.
pub async fn drop_db(mdb: &MongoDB) {
    mdb.db.drop().await.unwrap();
}
