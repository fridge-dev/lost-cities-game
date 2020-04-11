use crate::v2::task::events::{DbTaskEvent, WriteTargetTable};
use crate::local_disk_storage::sqlite_integration::SqliteWrapper;
use crate::local_disk_storage::sqlite_tables::{SqlGameData, SqlGameSummary};
use crate::v2::db_types::{DbGameSummary, DbError, DbGameData};
use crate::v2::db_api::DbResult;
use std::sync::mpsc::Receiver;
use tokio::sync::oneshot::Sender;
use std::fmt::Debug;
use std::convert::TryFrom;

/// Backend event loop of the async task model.
pub struct DatabaseBackendTask {
    receiver: Receiver<DbTaskEvent>,
    db_manager: DbManager,
}

impl DatabaseBackendTask {

    pub fn new(
        receiver: Receiver<DbTaskEvent>,
        sqlite: SqliteWrapper,
    ) -> Self {
        DatabaseBackendTask {
            receiver,
            db_manager: DbManager::new(sqlite),
        }
    }

    pub fn event_loop(self) {
        println!("INFO Starting DatabaseBackendTask event loop.");

        while let Ok(event) = self.receiver.recv() {
            self.handle_event(event);
        }

        println!("INFO Exiting DatabaseBackendTask event loop.");
    }

    /// Route the event to the correct DbManager method and send callback to client.
    fn handle_event(&self, event: DbTaskEvent) {
        match event {
            DbTaskEvent::Create(target_table, callback) => {
                match target_table {
                    WriteTargetTable::GameSummary(game_summary) => {
                        let result = self.db_manager.create_game_summary(game_summary);
                        DatabaseBackendTask::send(callback, result, "CreateGameSummary");
                    },
                    WriteTargetTable::GameData(game_data) => {
                        let result = self.db_manager.create_game_data(game_data);
                        DatabaseBackendTask::send(callback, result, "CreateGameData");
                    },
                }
            },
            DbTaskEvent::Update(target_table, callback) => {
                match target_table {
                    WriteTargetTable::GameSummary(game_summary) => {
                        let result = self.db_manager.update_game_summary(game_summary);
                        DatabaseBackendTask::send(callback, result, "UpdateGameSummary");
                    },
                    WriteTargetTable::GameData(game_data) => {
                        let result = self.db_manager.update_game_data(game_data);
                        DatabaseBackendTask::send(callback, result, "UpdateGameData");
                    },
                }
            },
            DbTaskEvent::GetGameSummary(game_id, callback) => {
                let result = self.db_manager.get_game_summary(game_id);
                DatabaseBackendTask::send(callback, result, "GetGameSummary");
            },
            DbTaskEvent::GetGameData(game_id, callback) => {
                let result = self.db_manager.get_game_data(game_id);
                DatabaseBackendTask::send(callback, result, "GetGameData");
            },
            DbTaskEvent::Archive(_) => {
                unimplemented!("Functionality to archive local DB file to S3");
            }
        }
    }

    fn send<T: Debug>(
        sender: Sender<Result<T, DbError>>,
        result: Result<T, DbError>,
        debug_message: &'static str,
    ) {
        if let Err(dropped_payload) = sender.send(result) {
            println!(
                "WARN: Failed to send DB result to callback channel for '{}'. Receiver probably dropped. Here's the dropped payload: {:?}",
                debug_message,
                dropped_payload,
            );
        }
    }
}

/// Biz logic
struct DbManager {
    sqlite: SqliteWrapper,
}

impl DbManager {

    pub fn new(sqlite: SqliteWrapper) -> Self {
        DbManager {
            sqlite
        }
    }

    pub fn create_game_summary(&self, game_summary: DbGameSummary) -> DbResult<()> {
        Ok(self.sqlite.insert_row(&SqlGameSummary::from(game_summary))?)
    }

    pub fn create_game_data(&self, game_data: DbGameData) -> DbResult<()> {
        Ok(self.sqlite.insert_row(&SqlGameData::from(game_data))?)
    }

    pub fn update_game_summary(&self, game_summary: DbGameSummary) -> DbResult<()> {
        Ok(self.sqlite.update_row(&SqlGameSummary::from(game_summary))?)
    }

    pub fn update_game_data(&self, game_data: DbGameData) -> DbResult<()> {
        Ok(self.sqlite.update_row(&SqlGameData::from(game_data))?)
    }

    pub fn get_game_summary(&self, game_id: String) -> DbResult<DbGameSummary> {
        let sql_game_summary = self.sqlite
            .select_row::<SqlGameSummary>(&game_id)?
            .ok_or(DbError::NotFound)?;

        DbGameSummary::try_from(sql_game_summary)
    }

    pub fn get_game_data(&self, game_id: String) -> DbResult<DbGameData> {
        let sql_game_data = self.sqlite
            .select_row::<SqlGameData>(&game_id)?
            .ok_or(DbError::NotFound)?;

        Ok(DbGameData::from(sql_game_data))
    }
}
