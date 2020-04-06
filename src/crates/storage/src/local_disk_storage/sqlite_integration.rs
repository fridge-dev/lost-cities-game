use rusqlite::{NO_PARAMS, Connection, OpenFlags, Row, ToSql};
use std::error::Error;
use std::path::Path;

type SqliteResult<T> = Result<T, Box<dyn Error>>;

/// Probably an unnecessary abstraction layer, but it helps me keep track of
/// all of the ways in which I integrate with sqlite.
pub struct SqliteWrapper {
    pub connection: Connection,
}

impl SqliteWrapper {
    pub fn connect<P: AsRef<Path>>(db_file_path: P) -> SqliteResult<Self> {
        let connection = Connection::open_with_flags(
            db_file_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                // https://www.sqlite.org/threadsafe.html:
                // SQLite can be safely used by multiple threads provided that
                // no single database connection is used simultaneously in two
                // or more threads
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
        )?;

        Ok(SqliteWrapper {
            connection
        })
    }

    pub fn create_table<R: SqlTableRow>(&self) -> SqliteResult<()> {
        self.connection.execute(R::table_create_statement(), NO_PARAMS)?;
        Ok(())
    }

    pub fn select_row<R: SqlTableRow>(&self, hash_key: &str) -> SqliteResult<Option<R>> {
        let mut statement = self.connection.prepare(&R::select_statement(hash_key))?;
        let mut row_results_iter = statement.query_map(NO_PARAMS, R::try_from_row)?;

        if let Some(row_result) = row_results_iter.next() {
            if row_results_iter.next().is_none() {
                Ok(Some(row_result?))
            } else {
                Err(format!(
                    "Multiple items returned for a SELECT with primary key. Key: {}",
                    hash_key
                ).into())
            }
        } else {
            Ok(None)
        }
    }

    pub fn insert_row<R: SqlTableRow>(&self, item: R) -> SqliteResult<()> {
        let statement_and_params = item.insert_statement_and_params();
        self.prepare_and_execute_named(statement_and_params, "INSERT")
    }

    pub fn update_row<R: SqlTableRow>(&self, item: &R) -> SqliteResult<()> {
        let statement_and_params = item.update_statement_and_params();
        self.prepare_and_execute_named(statement_and_params, "UPDATE")
    }

    fn prepare_and_execute_named(
        &self,
        statement_and_params: StatementAndParams<'_>,
        debug_message: &'static str
    ) -> SqliteResult<()> {
        let mut statement = self.connection.prepare(statement_and_params.sql_statement)?;
        let num_rows_changed = statement.execute_named(&statement_and_params.named_params)?;
        if num_rows_changed == 1 {
            Ok(())
        } else {
            Err(format!(
                "Successfully executed '{}', but changed '{}' rows when we expected to change only 1",
                debug_message,
                num_rows_changed,
            ).into())
        }
    }
}

/// For use with `prepare` and `execute_named`.
pub struct StatementAndParams<'a> {
    /// The `prepare` part of the sql command.
    pub sql_statement: &'static str,
    /// The `execute_named` part of the sql command.
    pub named_params: Vec<(&'static str, &'a dyn ToSql)>,
}

/// A struct that implements this represents one row in a SQLite table.
pub trait SqlTableRow {

    /// CREATE - Generate a SQL statement for creating this table.
    fn table_create_statement() -> &'static str;

    /// SELECT - Generate a SQL statement for selecting a single row from this table, given the hash key.
    fn select_statement(hash_key: &str) -> String;

    /// SELECT - Convert a sqlite Row type into this type.
    fn try_from_row(row: &Row<'_>) -> rusqlite::Result<Self> where Self: std::marker::Sized;

    /// INSERT - Generate a SQL statement for inserting this item into the table with a named query.
    fn insert_statement_and_params(&self) -> StatementAndParams;

    /// UPDATE - Generate a SQL statement for updating a single row with a named query.
    fn update_statement_and_params(&self) -> StatementAndParams;
}
