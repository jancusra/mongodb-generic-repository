//! Integration tests for database entity Note

#![allow(unused_imports)]
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;

use crate::database::repository::MongoDB;
use crate::test::common::{drop_db, test_db};
use crate::test::entity::note::Note;

/// Insert the five example notes into the given (isolated) database.
async fn seed_notes(mdb: &MongoDB) {
    let notes = [
        Note::example(&ObjectId::new()),
        Note::example2(&ObjectId::new()),
        Note::example3(&ObjectId::new()),
        Note::example4(&ObjectId::new()),
        Note::example5(&ObjectId::new()),
    ];

    for note in &notes {
        mdb.create_document(note).await.unwrap();
    }
}

#[tokio::test]
async fn create_database_document() {
    let mdb = test_db().await;
    let new_note_id = ObjectId::new();
    let new_note = Note::example(&new_note_id);

    let result = mdb.create_document(&new_note).await.unwrap();

    drop_db(&mdb).await;

    assert_eq!(new_note_id, result.inserted_id.as_object_id().unwrap());
}

#[tokio::test]
async fn create_and_update_database_document() {
    let mdb = test_db().await;
    let new_note_id = ObjectId::new();
    let mut new_note = Note::example(&new_note_id);

    let create_result = mdb.create_document(&new_note).await.unwrap();

    new_note.title = "Barcelona trip".to_string();
    new_note.content = "Cycling trip around Barcelona".to_string();

    let update_result = mdb
        .update_document::<Note>(&new_note_id, &new_note)
        .await
        .unwrap();

    drop_db(&mdb).await;

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
    let mdb = test_db().await;
    let new_note_id = ObjectId::new();
    let new_note = Note::example(&new_note_id);

    let create_result = mdb.create_document(&new_note).await.unwrap();
    let delete_result = mdb.delete_document::<Note>(&new_note_id).await.unwrap();

    drop_db(&mdb).await;

    assert_eq!(
        (new_note_id, 1),
        (
            create_result.inserted_id.as_object_id().unwrap(),
            delete_result.deleted_count
        )
    );
}

#[tokio::test]
async fn get_all_with_limit() {
    let mdb = test_db().await;
    seed_notes(&mdb).await;

    let all = mdb.get_all::<Note>().await.unwrap();
    let limited = mdb
        .get_all_with_options::<Note>(None, Some(2), None, None)
        .await
        .unwrap();

    drop_db(&mdb).await;

    assert_eq!((5, 2), (all.len(), limited.len()));
}

#[tokio::test]
async fn get_all_with_skip() {
    let mdb = test_db().await;
    seed_notes(&mdb).await;

    // 5 documents total, skipping the first 3 leaves 2
    let skipped = mdb
        .get_all_with_options::<Note>(None, None, Some(3), None)
        .await
        .unwrap();

    drop_db(&mdb).await;

    assert_eq!(2, skipped.len());
}

#[tokio::test]
async fn get_all_with_sort() {
    let mdb = test_db().await;
    seed_notes(&mdb).await;

    let sorted = mdb
        .get_all_with_options::<Note>(None, None, None, Some(doc! { "title": 1 }))
        .await
        .unwrap();

    let titles: Vec<&str> = sorted.iter().map(|note| note.title.as_str()).collect();

    drop_db(&mdb).await;

    assert_eq!(
        vec![
            "Alps hike",
            "Barcelona trip",
            "Cycling trip",
            "Danube cruise",
            "Everest base camp",
        ],
        titles
    );
}

#[tokio::test]
async fn get_all_with_pagination() {
    let mdb = test_db().await;
    seed_notes(&mdb).await;

    // sorted by title ascending, skip the first 2 and take the next 2
    let page = mdb
        .get_all_with_options::<Note>(None, Some(2), Some(2), Some(doc! { "title": 1 }))
        .await
        .unwrap();

    let titles: Vec<&str> = page.iter().map(|note| note.title.as_str()).collect();

    drop_db(&mdb).await;

    assert_eq!(vec!["Cycling trip", "Danube cruise"], titles);
}
