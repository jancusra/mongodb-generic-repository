//! Integration tests for database entity Note

#![allow(unused_imports)]
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;

use crate::database::repository::MongoDB;
use crate::test::entity::note::Note;

#[tokio::test]
async fn create_database_document() {
    let mdb = MongoDB::new().await.unwrap();
    let new_note_id = ObjectId::new();
    let new_note = Note::example(&new_note_id);

    let result = mdb.create_document(&new_note).await.unwrap();

    assert_eq!(new_note_id, result.inserted_id.as_object_id().unwrap());
}

#[tokio::test]
async fn create_and_update_database_document() {
    let mdb = MongoDB::new().await.unwrap();
    let new_note_id = ObjectId::new();
    let mut new_note = Note::example(&new_note_id);

    let create_result = mdb.create_document(&new_note).await.unwrap();

    new_note.title = "Barcelona trip".to_string();
    new_note.content = "Cycling trip around Barcelona".to_string();

    let update_result = mdb
        .update_document::<Note>(&new_note_id, &new_note)
        .await
        .unwrap();

    assert_eq!(
        (new_note_id, 1),
        (
            create_result.inserted_id.as_object_id().unwrap(),
            update_result.modified_count
        )
    );
}

#[tokio::test]
async fn create_and_delete_database_document() {
    let mdb = MongoDB::new().await.unwrap();
    let new_note_id = ObjectId::new();
    let new_note = Note::example(&new_note_id);

    let create_result = mdb.create_document(&new_note).await.unwrap();
    let delete_result = mdb.delete_document::<Note>(&new_note_id).await.unwrap();

    assert_eq!(
        (new_note_id, 1),
        (
            create_result.inserted_id.as_object_id().unwrap(),
            delete_result.deleted_count
        )
    );
}
