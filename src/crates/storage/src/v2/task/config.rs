use crate::local_disk_storage::sqlite_integration::SqliteWrapper;
use crate::v2::task::receiver::DatabaseBackendTask;
use crate::v2::task::sender::DatabaseClient;
use tokio::sync::mpsc;
use tokio::task;

pub(crate) fn start_database_task(sqlite: SqliteWrapper) -> DatabaseClient {
    let (tx, rx) = mpsc::unbounded_channel();

    let backend_task = DatabaseBackendTask::new(rx, sqlite);

//    let mut runtime = tokio::runtime::Builder::new()
//        .core_threads(1)
//        .max_threads(1)
//        .build()
//        .unwrap();

    task::spawn_blocking(|| {
        backend_task.event_loop()
    });

    DatabaseClient::new(tx)
}
