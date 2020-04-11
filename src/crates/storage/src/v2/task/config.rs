use crate::local_disk_storage::sqlite_integration::SqliteWrapper;
use crate::v2::task::receiver::DatabaseBackendTask;
use crate::v2::task::sender::DatabaseClient;
use std::sync::mpsc as std_mpsc;
use std::thread;

pub(crate) fn start_database_task(sqlite: SqliteWrapper) -> DatabaseClient {
    let (tx, rx) = std_mpsc::channel();

    let backend_task = DatabaseBackendTask::new(rx, sqlite);

    thread::spawn(|| {
        backend_task.event_loop()
    });

    DatabaseClient::new(tx)
}
