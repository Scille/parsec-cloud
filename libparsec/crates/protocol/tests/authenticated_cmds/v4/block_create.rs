// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::duplicate_mod)]

use super::authenticated_cmds;

// The compat mod is defined to re-use tests from v3
// Loading a file as a module more than once causes it to be compiled multiple
// times, taking longer and putting duplicate content into the module tree.
// In this case, this is exactly what we want.
#[path = "../v3/block_create.rs"]
mod compat;

// Request

pub use compat::req;

// Responses

pub use compat::rep_ok;

pub use compat::rep_already_exists;

pub use compat::rep_not_found;

pub use compat::rep_not_allowed;

pub use compat::rep_in_maintenance;

pub use compat::rep_timeout;
