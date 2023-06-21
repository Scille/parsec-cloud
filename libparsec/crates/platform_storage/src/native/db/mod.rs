// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod error;
mod executor;
mod local_database;
mod option;
mod types;

#[cfg(feature = "test-with-testbed")]
mod testbed;

pub use error::{DatabaseError, DatabaseResult};
pub use local_database::{LocalDatabase, VacuumMode, LOCAL_DATABASE_MAX_VARIABLE_NUMBER};
pub use option::AutoVacuum;
pub use types::{CoalesceTotalSize, DateTime};
