use crate::v2::db_api::GameDatabase;
use crate::v2::task;
use crate::local_disk_storage::sqlite_integration;
use crate::local_disk_storage::sqlite_integration::SqliteWrapper;
use std::error::Error;
use std::sync::Arc;

pub enum DatabaseMode {
    Prod,
    Test(String),
}

impl DatabaseMode {
    fn db_file_location(&self) -> &str {
        match self {
            DatabaseMode::Prod => "/tmp/frj-game.prod.db",
            DatabaseMode::Test(file) => file,
        }
    }
}

pub fn connect_to_database(mode: DatabaseMode) -> Result<
    Arc<dyn GameDatabase + Send + Sync>,
    Box<dyn Error>
> {
    let sqlite = SqliteWrapper::connect(mode.db_file_location())?;

    // Conditionally create tables
    sqlite_integration::create_all_tables(&sqlite)?;

    Ok(Arc::new(task::config::start_database_task(sqlite)))
}