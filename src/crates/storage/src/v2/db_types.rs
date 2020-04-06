use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct DbGameSummary {
    pub game_id: String,
    pub game_creation_time_sec: u32,
    pub game_type: DbGameType,
    pub game_status: DbGameStatus,
    pub game_summary_blob_opt: Option<Vec<u8>>
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DbGameType {
    LostCities,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DbGameStatus {
    WaitingForPlayers,
    InProgress,
    Completed,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DbGameData {
    pub game_id: String,
    pub game_data_blob: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum DbError {

    // ================ Client fault ================

    /// Resource doesn't exist. Caller is assumed to know which resource they were querying for.
    NotFound,

    /// Duplicate creation of resource.
    AlreadyExists,

    // ================ Server fault ================

    /// Backend caused an error
    Internal(DbErrorCause),
}

impl Error for DbError {}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DbError::NotFound => write!(f, "Resource not found in storage layer."),
            DbError::AlreadyExists => write!(f, "Resource already exists in storage layer."),
            DbError::Internal(cause) => write!(f, "Internal error in storage layer: {}", cause),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DbErrorCause {
    /// Internal (my code) logic error
    Internal(&'static str),

    MalformedData(String),

    /// Dependent service/database error.
    Sqlite(rusqlite::Error),
    //S3(rusoto::Error),
}

impl Display for DbErrorCause {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DbErrorCause::Internal(msg) => write!(f, "Internal logic error: '{}'", msg),
            DbErrorCause::MalformedData(msg) => write!(f, "Application doesn't recognize the persisted data: {}", msg),
            DbErrorCause::Sqlite(sqlite_error) => write!(f, "Sqlite error: {}", sqlite_error),
        }
    }
}
