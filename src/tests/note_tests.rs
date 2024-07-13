//! Unit tests for database entity Note

#![allow(unused_imports)]
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;

use mongodb_repo::database::entity_note::Note;
use mongodb_repo::database::repository::MongoDB;

#[tokio::test]
async fn create_database_document() {
    let mdb = MongoDB::new().await;
    let new_note_id = ObjectId::new();
    let new_note = Note::example(&new_note_id);

    let result = mdb.create_document(&new_note).await;

    assert_eq!(new_note_id, result.unwrap());
}

#[tokio::test]
async fn create_and_update_database_document() {
    let mdb = MongoDB::new().await;
    let new_note_id = ObjectId::new();
    let mut new_note = Note::example(&new_note_id);

    let create_result = mdb.create_document(&new_note).await;

    new_note.title = "Barcelona trip".to_string();
    new_note.content = "Cycling trip around Barcelona".to_string();

    let update_result = mdb.update_document::<Note>(&new_note_id, &new_note).await;

    assert_eq!((new_note_id, 1), (create_result.unwrap(), update_result.unwrap().modified_count));
}

#[tokio::test]
async fn create_and_delete_database_document() {
    let mdb = MongoDB::new().await;
    let new_note_id = ObjectId::new();
    let new_note = Note::example(&new_note_id);

    let create_result = mdb.create_document(&new_note).await;
    let delete_result = mdb.delete_document::<Note>(&new_note_id).await;

    assert_eq!((new_note_id, 1), (create_result.unwrap(), delete_result.unwrap().deleted_count));
}