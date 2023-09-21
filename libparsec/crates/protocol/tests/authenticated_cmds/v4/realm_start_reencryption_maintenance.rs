// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::duplicate_mod)]

// The compat module allows to re-use tests from previous major API
#[path = "../v3/realm_start_reencryption_maintenance.rs"]
mod compat;

use super::authenticated_cmds;

// Request

pub use compat::req;

// Responses

pub use compat::rep_ok;

pub use compat::rep_not_allowed;

pub use compat::rep_not_found;

pub use compat::rep_bad_encryption_revision;

pub use compat::rep_participant_mismatch;

pub use compat::rep_maintenance_error;

pub use compat::rep_in_maintenance;

pub use compat::rep_bad_timestamp;
