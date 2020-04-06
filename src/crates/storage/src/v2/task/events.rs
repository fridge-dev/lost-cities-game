use crate::v2::db_types::{DbGameSummary, DbGameData, DbError};
use tokio::sync::oneshot::Sender;

pub type AsyncCallback<T> = Sender<Result<T, DbError>>;

pub enum DbTaskEvent {
    Create(WriteTargetTable, AsyncCallback<()>),
    Update(WriteTargetTable, AsyncCallback<()>),
    GetGameSummary(String, AsyncCallback<DbGameSummary>),
    GetGameData(String, AsyncCallback<DbGameData>),
    #[allow(dead_code)]
    Archive(ArchivalConfig),
}

pub enum WriteTargetTable {
    GameSummary(DbGameSummary),
    GameData(DbGameData),
}

pub struct ArchivalConfig {
    // This use case isn't fleshed out yet, but I want to periodically
    // archive the DB to S3. If any config is required, it would go hear.
}
