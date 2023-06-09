// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod error;
mod executor;
mod local_database;
mod option;
mod types;

// We have two ways of mocking database:
// - `test-in-memory-mock`: the legacy global switch (should only be used with the Python tests)
// - `test-with-testbed`: the faster and parallel system (which uses configuration dir as discriminant)
#[cfg(feature = "test-in-memory-mock")]
mod in_memory_mock;
#[cfg(feature = "test-with-testbed")]
mod testbed;

pub use error::{DatabaseError, DatabaseResult};
#[cfg(feature = "test-in-memory-mock")]
pub use in_memory_mock::{test_clear_local_db_in_memory_mock, test_toggle_local_db_in_memory_mock};
pub use local_database::{LocalDatabase, VacuumMode, LOCAL_DATABASE_MAX_VARIABLE_NUMBER};
pub use option::AutoVacuum;
pub use types::{CoalesceTotalSize, DateTime};
