use rusqlite::{Row, ToSql};
use crate::local_disk_storage::sqlite_integration::{SqlTableRow, StatementAndParams};

#[derive(Debug, PartialEq, Clone)]
pub struct SqlGameSummary {
    pub game_id: String,
    pub game_creation_time_sec: u32,
    pub game_type: u8,
    pub game_status: u8,
    pub game_summary_blob_opt: Option<Vec<u8>>
}

impl SqlGameSummary {
    fn as_named_params<'a>(&'a self) -> Vec<(&'static str, &'a dyn ToSql)> {
        let mut params: Vec<(&'static str, &'a dyn ToSql)> = Vec::with_capacity(5);
        params.push((":game_id", &self.game_id));
        params.push((":game_creation_time_sec", &self.game_creation_time_sec));
        params.push((":game_type", &self.game_type));
        params.push((":game_status", &self.game_status));

        if let Some(game_summary_blob) = &self.game_summary_blob_opt {
            params.push((":game_summary_blob", game_summary_blob));
        }

        params
    }
}

impl SqlTableRow for SqlGameSummary {
    fn table_create_statement() -> &'static str {
        "CREATE TABLE IF NOT EXISTS game_summary ( \
            game_id TEXT PRIMARY KEY, \
            game_creation_time_sec INTEGER NOT NULL, \
            game_type INTEGER NOT NULL, \
            game_status INTEGER NOT NULL, \
            game_summary_blob BLOB \
        )"
    }

    fn select_statement(game_id: &str) -> String {
        format!(
            "SELECT game_id, game_creation_time_sec, game_type, game_status, game_summary_blob \
                FROM game_summary \
                WHERE game_id = '{}'",
            game_id
        )
    }

    fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(SqlGameSummary {
            game_id: row.get("game_id")?,
            game_creation_time_sec: row.get("game_creation_time_sec")?,
            game_type: row.get("game_type")?,
            game_status: row.get("game_status")?,
            game_summary_blob_opt: row.get("game_summary_blob")?,
        })
    }

    fn insert_statement_and_params(&self) -> StatementAndParams {
        let sql_statement = "\
            INSERT INTO game_summary \
            (game_id, game_creation_time_sec, game_type, game_status, game_summary_blob) VALUES \
            (:game_id, :game_creation_time_sec, :game_type, :game_status, :game_summary_blob) \
        ";

        StatementAndParams {
            sql_statement,
            named_params: self.as_named_params()
        }
    }

    fn update_statement_and_params(&self) -> StatementAndParams {
        let sql_statement = "\
            UPDATE game_summary \
            SET \
                game_creation_time_sec = :game_creation_time_sec, \
                game_type = :game_type, \
                game_status = :game_status, \
                game_summary_blob = :game_summary_blob \
            WHERE game_id = :game_id \
        ";

        StatementAndParams {
            sql_statement,
            named_params: self.as_named_params()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SqlGameData {
    pub game_id: String,
    pub game_data_blob: Vec<u8>,
}

impl SqlGameData {
    fn as_named_params<'a>(&'a self) -> Vec<(&'static str, &'a dyn ToSql)> {
        vec![
            (":game_id", &self.game_id),
            (":game_data_blob", &self.game_data_blob),
        ]
    }
}

impl SqlTableRow for SqlGameData {
    fn table_create_statement() -> &'static str {
        "CREATE TABLE IF NOT EXISTS game_data ( \
            game_id TEXT PRIMARY KEY, \
            game_data_blob BLOB NOT NULL \
        )"
    }

    fn select_statement(game_id: &str) -> String {
        format!(
            "SELECT game_id, game_data_blob \
                FROM game_data \
                WHERE game_id = '{}'",
            game_id
        )
    }

    fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(SqlGameData {
            game_id: row.get("game_id")?,
            game_data_blob: row.get("game_data_blob")?,
        })
    }

    fn insert_statement_and_params(&self) -> StatementAndParams {
        let sql_statement = "\
            INSERT INTO game_data \
            (game_id, game_data_blob) VALUES \
            (:game_id, :game_data_blob) \
        ";

        StatementAndParams {
            sql_statement,
            named_params: self.as_named_params()
        }
    }

    fn update_statement_and_params(&self) -> StatementAndParams {
        let sql_statement = "\
            UPDATE game_data \
            SET \
                game_data_blob = :game_data_blob \
            WHERE game_id = :game_id \
        ";

        StatementAndParams {
            sql_statement,
            named_params: self.as_named_params()
        }
    }
}
