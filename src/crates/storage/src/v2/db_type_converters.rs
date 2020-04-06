// ------- Game Data -------
// Application layer: DbGameData
// Storage layer: SqlGameData

use crate::v2::db_types::{DbGameData, DbGameSummary, DbError, DbGameType, DbGameStatus, DbErrorCause};
use crate::local_disk_storage::sqlite_tables::{SqlGameData, SqlGameSummary};
use std::convert::TryFrom;

impl From<SqlGameData> for DbGameData {
    fn from(sql_game_data: SqlGameData) -> Self {
        DbGameData {
            game_id: sql_game_data.game_id,
            game_data_blob: sql_game_data.game_data_blob,
        }
    }
}

impl From<DbGameData> for SqlGameData {
    fn from(db_game_data: DbGameData) -> Self {
        SqlGameData {
            game_id: db_game_data.game_id,
            game_data_blob: db_game_data.game_data_blob,
        }
    }
}

// ------- Game Summary -------
// Application layer: DbGameSummary
// Storage layer: SqlGameSummary

impl TryFrom<SqlGameSummary> for DbGameSummary {
    type Error = DbError;

    fn try_from(sql_game_summary: SqlGameSummary) -> Result<Self, Self::Error> {
        Ok(DbGameSummary {
            game_id: sql_game_summary.game_id,
            game_creation_time_sec: sql_game_summary.game_creation_time_sec,
            game_type: DbGameType::try_from(sql_game_summary.game_type)?,
            game_status: DbGameStatus::try_from(sql_game_summary.game_status)?,
            game_summary_blob_opt: sql_game_summary.game_summary_blob_opt,
        })
    }
}

impl From<DbGameSummary> for SqlGameSummary {
    fn from(db_game_summary: DbGameSummary) -> Self {
        SqlGameSummary {
            game_id: db_game_summary.game_id,
            game_creation_time_sec: db_game_summary.game_creation_time_sec,
            game_type: db_game_summary.game_type.into(),
            game_status: db_game_summary.game_status.into(),
            game_summary_blob_opt: db_game_summary.game_summary_blob_opt,
        }
    }
}

// ------- Game Type -------
// Application layer: DbGameType
// Storage layer: u8

impl From<DbGameType> for u8 {
    fn from(db_game_type: DbGameType) -> Self {
        match db_game_type {
            DbGameType::LostCities => 1,
        }
    }
}

impl TryFrom<u8> for DbGameType {
    type Error = DbError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DbGameType::LostCities),
            _ => Err(DbError::Internal(DbErrorCause::MalformedData(
                format!("Unexpected DbGameType u32 value {}", value)
            )))
        }
    }
}

// ------- Game Status -------
// Application layer: DbGameStatus
// Storage layer: u8

impl From<DbGameStatus> for u8 {
    fn from(db_game_status: DbGameStatus) -> Self {
        match db_game_status {
            DbGameStatus::WaitingForPlayers => 1,
            DbGameStatus::InProgress => 2,
            DbGameStatus::Completed => 3,
        }
    }
}

impl TryFrom<u8> for DbGameStatus {
    type Error = DbError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DbGameStatus::WaitingForPlayers),
            2 => Ok(DbGameStatus::InProgress),
            3 => Ok(DbGameStatus::Completed),
            _ => Err(DbError::Internal(DbErrorCause::MalformedData(
                format!("Unexpected DbGameStatus u32 value {}", value)
            )))
        }
    }
}

// ------- Errors -------
// Application layer: DbError
// Storage layer: (rusqlite::Error, ...)

impl From<rusqlite::Error> for DbError {
    fn from(sqlite_error: rusqlite::Error) -> Self {
        DbError::Internal(DbErrorCause::Sqlite(sqlite_error))
    }
}
