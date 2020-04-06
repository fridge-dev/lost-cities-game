use crate::local_disk_storage::sqlite_integration::{SqliteWrapper, create_all_tables};
use crate::local_disk_storage::sqlite_tables::{SqlGameSummary, SqlGameData};
use crate::test_utils::{TestFileHandle, rand_str};

#[test]
fn create_tables_idempotent() {
    let db_file = TestFileHandle::new(format!("./frj-game-{}.db", rand_str()));
    db_file.rm("before");
    let sqlite = SqliteWrapper::connect(&db_file.file_path).expect("SqliteWrapper::create");

    create_all_tables(&sqlite).expect("create_all_tables1");
    create_all_tables(&sqlite).expect("create_all_tables2");
    create_all_tables(&sqlite).expect("create_all_tables3");
}

#[test]
fn test_accessing_sql_game_summary() {
    // Setup
    let db_file = TestFileHandle::new(format!("./frj-game-{}.db", rand_str()));
    db_file.rm("before");
    let sqlite = SqliteWrapper::connect(&db_file.file_path).expect("SqliteWrapper::create");
    create_all_tables(&sqlite).expect("create_all_tables");
    let game_id: String = rand_str();

    // INSERT
    let obj_wrote1 = SqlGameSummary {
        game_id: game_id.clone(),
        game_creation_time_sec: 134123412,
        game_type: 1,
        game_status: 2,
        game_summary_blob_opt: None
    };
    sqlite.insert_row(&obj_wrote1).expect("insert_row");

    // SELECT
    let obj_read1 = sqlite.select_row(&game_id)
        .expect("select_row failed")
        .expect("select_row found no row");
    assert_eq!(obj_read1, obj_wrote1);

    // UPDATE
    let mut obj_wrote2 = obj_wrote1;
    obj_wrote2.game_status += 1;
    obj_wrote2.game_summary_blob_opt = Some(vec![123, 234, 12, 23, 45]);
    sqlite.update_row(&obj_wrote2).expect("update_row");

    // SELECT
    let obj_read2 = sqlite.select_row::<SqlGameSummary>(&game_id)
        .expect("select_row failed")
        .expect("select_row found no row");
    assert_eq!(obj_read2, obj_wrote2);
    assert_ne!(obj_read2, obj_read1);
}

#[test]
fn test_accessing_sql_game_data() {
    // Setup
    let db_file = TestFileHandle::new(format!("./frj-game-{}.db", rand_str()));
    db_file.rm("before");
    let sqlite = SqliteWrapper::connect(&db_file.file_path).expect("SqliteWrapper::create");
    create_all_tables(&sqlite).expect("create_all_tables");
    let game_id: String = rand_str();

    // INSERT
    let obj_wrote1 = SqlGameData {
        game_id: game_id.clone(),
        game_data_blob: vec![11, 22, 33, 44, 55],
    };
    sqlite.insert_row(&obj_wrote1).expect("insert_row");

    // SELECT
    let obj_read1 = sqlite.select_row(&game_id)
        .expect("select_row failed")
        .expect("select_row found no row");
    assert_eq!(obj_read1, obj_wrote1);

    // UPDATE
    let mut obj_wrote2 = obj_wrote1;
    obj_wrote2.game_data_blob = vec![123, 234, 12, 23, 45, 53, 32, 123];
    sqlite.update_row(&obj_wrote2).expect("update_row");

    // SELECT
    let obj_read2 = sqlite.select_row::<SqlGameData>(&game_id)
        .expect("select_row failed")
        .expect("select_row found no row");
    assert_eq!(obj_read2, obj_wrote2);
    assert_ne!(obj_read2, obj_read1);
}
