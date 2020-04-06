use crate::v2::config::{connect_to_database, DatabaseMode};
use crate::test_utils::{TestFileHandle, rand_str};
use crate::v2::db_types::{DbGameData, DbError};
use tokio::task;
use tokio::sync::oneshot;

#[test]
fn ignore_intellij_hack() {
    // My IntelliJ can only run tests if at least
    // 1 non-tokio::test exists in a file.
}

#[tokio::test(threaded_scheduler)]
async fn database_integration_test() {
    // == setup ==
    let db_file = TestFileHandle::new(format!("./safe-to-delete.test-{}.db", rand_str()));
    let game_id = "test-ff98h1fj2fo4";

    let db_client1 = connect_to_database(DatabaseMode::Test(db_file.file_path.clone()))
        .expect("connect_to_database");
    let db_client2 = db_client1.clone();

    // == execute & verify ==

    // 1. Load (nothing)
    let load_before_write_result = db_client1.load_game_data(game_id.to_owned()).await;
    assert_eq!(load_before_write_result, Err(DbError::NotFound));

    // 2. Write (in another task)
    let (write_notifier_tx, mut write_notifier_rx) = oneshot::channel::<Result<(), DbError>>();
    task::spawn(async move {
        let game_data_to_write = DbGameData {
            game_id: game_id.to_owned(),
            game_data_blob: vec![1, 2, 3, 4, 5],
        };

        let write_result = db_client2.create_game_data(game_data_to_write).await;
        let _result = write_notifier_tx.send(write_result);
    });

    // 3. Load (data)
    let load_after_write = db_client1.load_game_data(game_id.to_owned())
        .await
        .expect("load_game_data after write");
    assert_eq!(load_after_write, DbGameData {
        game_id: game_id.to_owned(),
        game_data_blob: vec![1, 2, 3, 4, 5],
    });

    // Verify that the write from earlier succeeded to notify client
    assert_eq!(write_notifier_rx.try_recv(), Ok(Ok(())));
}
