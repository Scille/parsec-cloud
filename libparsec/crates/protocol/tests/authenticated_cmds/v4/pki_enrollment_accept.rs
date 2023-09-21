// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::duplicate_mod)]

// The compat module allows to re-use tests from previous major API
#[path = "../v3/pki_enrollment_accept.rs"]
mod compat;

use super::authenticated_cmds;

// Request

pub use compat::req;

// Responses

pub use compat::rep_ok;

pub use compat::rep_not_allowed;

pub use compat::rep_invalid_payload_data;

pub use compat::rep_invalid_certification;

pub use compat::rep_invalid_data;

pub use compat::rep_not_found;

pub use compat::rep_no_longer_available;

pub use compat::rep_already_exists;

pub use compat::rep_active_users_limit_reached;

pub use compat::rep_bad_timestamp;

pub use compat::rep_require_greater_timestamp;
