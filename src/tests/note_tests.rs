#![allow(unused_imports)]
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;

use mongodb_repo::database::entity_note::Note;
use mongodb_repo::database::repository::MongoDB;

#[tokio::test]
async fn create_and_get_document_by_id() {
    let mdb = MongoDB::new().await;
    let note_id = "65b47748cd379327809001f9".to_string();
    let new_note_id = ObjectId::from_str(&note_id).unwrap();
    let new_note = Note::example(&new_note_id);

    let create_result = mdb.create_document(&new_note).await;
    let get_id_result = mdb.get_by_id::<Note>(&note_id).await;

    assert_eq!((new_note_id, Note::example(&new_note_id)), (create_result.unwrap(), get_id_result.unwrap()));

    mdb.delete_document::<Note>(&new_note_id).await;
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