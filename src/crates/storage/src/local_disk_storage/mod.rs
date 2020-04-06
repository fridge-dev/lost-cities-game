//! == Current planned server + DB architecture ==
//!
//! For scale up to ~10k concurrent active users (conservative estimate), the entire backend run
//! on a single server instance. This assumption is based on this being a hobby project and I don't
//! expect it to reach such a substantial load. The architecture will have only one bottleneck on
//! the horizontal scalability, which is that it will use a local no-sql database. This is to
//! minimize costs for a project which I will not make any profit on. I will design the database
//! layer in such a way that if I'd like, I can replace the local database with an external,
//! distributed database (probably DynamoDB), but keep the same interface with the application layer.
//! This means that even if we use a relational database on disk, we should expose a no-sql like
//! interface.
//!
//! **There will be 3 layers of "persistence"**:
//!
//! 1. In-memory of server process
//! 2. Disk-backed database (e.g. sqlite)
//! 3. External archives (e.g. S3, DynamoDB)
//!
//! All game state will be stored in process's memory and flushed periodically (e.g. 5 min) to a
//! disk-backed-database (sqlite). Even more infrequently (e.g. 1-hour), there will be a job which
//! uploads the disk-backed-db to an external storage (S3). There may also be activity based writes
//! to external storage (write to DynamoDB upon game completion).
//!
//! The main active persistence will be held in memory for performance reasons (no shared lock on
//! DB writes) and because the data is not critical (users' quality of life won't be impacted if
//! data is lost). Periodic flushes of LRU in-memory data to disk will help ensure some amount of
//! durability and give the opportunity the keep persistent state between processes (allows to deploy
//! changes to server). Periodic archival of data off-host will help ensure some amount of resilience
//! in case of EC2 host failure.
//!
//! === Tables ===
//!
//! There will be two tables:
//!
//! 1. `GameSummary`
//!     * This table is for infrequently mutated, infrequently accessed, "static"-like data.
//!     * This table will hold metadata about the game like the players involved and high level status.
//!     * This table can optionally hold a game-specific metadata blob.
//! 2. `GameData`
//!     * This table is for high-mutation data. This table will likely be fronted by a two-way HTTP2 stream between client and server.
//!     * This table will hold a game-specific data blob used to store state which is updated on a "per-turn" basis.
//!     * It is not expected for this table to be accessed after a game is complete or before it has started.
pub(crate) mod sqlite_integration;
pub(crate) mod sqlite_tables;
#[cfg(test)]
mod sqlite_tests;
