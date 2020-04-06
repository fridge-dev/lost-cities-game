use crate::local_disk_storage::sqlite_integration::{SqliteWrapper, SqlTableRow};
use crate::local_disk_storage::sqlite_tables::{SqlGameSummary, SqlGameData};
use std::fs;
use std::io::ErrorKind;
use std::error::Error;

pub struct TestFileHandle {
    pub file_path: String,
}

impl TestFileHandle {
    pub fn new(file_path: String) -> Self {
        TestFileHandle { file_path }
    }

    pub fn rm(&self, panic_msg: &str) {
        match fs::remove_file(&self.file_path) {
            Ok(_) => {},
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {},
                _ => panic!("fs::remove_file failed - {}: Debug={:?} Display={}", panic_msg, e, e)
            },
        }
    }
}

impl Drop for TestFileHandle {
    fn drop(&mut self) {
        self.rm("drop");
    }
}

fn create_all_tables(sqlite_wrapper: &SqliteWrapper) -> Result<(), Box<dyn Error>> {
    sqlite_wrapper.create_table::<SqlGameSummary>()?;
    sqlite_wrapper.create_table::<SqlGameData>()?;

    Ok(())
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
    sqlite.insert_row(obj_wrote1.clone()).expect("insert_row");

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
    sqlite.insert_row(obj_wrote1.clone()).expect("insert_row");

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

#[test]
fn query_name_example() {
    // Setup
    let db_file = TestFileHandle::new(format!("./frj-game-{}.db", rand_str()));
    db_file.rm("before");
    let sqlite = SqliteWrapper::connect(&db_file.file_path).expect("SqliteWrapper::create");
    create_all_tables(&sqlite).expect("create_all_tables");
    let game_id: String = rand_str();

    // INSERT
    let sql_game_summary = SqlGameSummary {
        game_id: game_id.clone(),
        game_creation_time_sec: 134123412,
        game_type: 1,
        game_status: 2,
        game_summary_blob_opt: None
    };
    sqlite.insert_row(sql_game_summary.clone()).expect("insert_row");

    // SELECT
    let select_sql = "SELECT game_id, game_creation_time_sec, game_type, game_status, game_summary_blob \
        FROM game_summary \
        WHERE game_id = :game_id";
    let mut statement = sqlite.connection.prepare(select_sql).expect("prepare_select");
    {
        let row_results_iter = statement.query_map_named(
            rusqlite::named_params!{
                    ":game_id": game_id,
                },
            SqlGameSummary::try_from_row
        ).expect("query_map_named");

        let mut rows = Vec::with_capacity(1);
        for row_result in row_results_iter {
            rows.push(row_result.expect("row mapping"));
        }

        println!("{:?}", rows);
        assert_eq!(rows.len(), 1);
    }

    // SELECT other
    {
        let mut row_results_iter = statement.query_map_named(
            rusqlite::named_params!{
                    ":game_id": "other"
                },
            SqlGameSummary::try_from_row
        ).expect("query_map_named2");

        if let Some(row_result) = row_results_iter.next() {
            panic!("Found row Result {:?} but expect no rows to exist.", row_result);
        }
    }
}

fn rand_str() -> String {
    format!("{:x}", rand::random::<u64>())
}
