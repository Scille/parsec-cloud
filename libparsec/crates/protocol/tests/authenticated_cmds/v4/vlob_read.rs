// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::duplicate_mod)]

use super::authenticated_cmds;

// The compat module allows to re-use tests from previous major API
#[path = "../v3/vlob_read.rs"]
mod compat;

// Request

pub use compat::req;

// Responses

pub fn rep_ok() {
    // TODO #4545: Implement test
}

pub use compat::rep_not_found;

pub use compat::rep_not_allowed;

pub use compat::rep_bad_version;

pub use compat::rep_bad_encryption_revision;

pub use compat::rep_in_maintenance;
