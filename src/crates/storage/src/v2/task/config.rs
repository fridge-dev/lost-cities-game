use crate::local_disk_storage::sqlite_integration::SqliteWrapper;
use crate::v2::task::receiver::DatabaseBackendTask;
use crate::v2::task::sender::DatabaseClient;
use crossbeam::channel;
use std::thread;

pub(crate) fn start_database_task(sqlite: SqliteWrapper) -> DatabaseClient {
    let (tx, rx) = channel::unbounded();

    let backend_task = DatabaseBackendTask::new(rx, sqlite);

    thread::spawn(|| {
        backend_task.event_loop()
    });

    DatabaseClient::new(tx)
}
