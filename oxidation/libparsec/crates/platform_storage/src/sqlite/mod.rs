// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

//! Implementation of the storage using an sqlite driver

mod cache;
mod chunk_storage_auto_impl;
mod data;
mod db;
mod error;
mod sql_types;
mod tables;

pub use cache::SqliteCacheStorage;
pub use data::SqliteDataStorage;
#[cfg(feature = "test-in-memory-mock")]
pub use db::{test_clear_local_db_in_memory_mock, test_toggle_local_db_in_memory_mock};
pub use db::{AutoVacuum, LocalDatabase, VacuumMode};

#[cfg(test)]
#[path = "../../tests/unit/sqlite/mod.rs"]
mod tests;
