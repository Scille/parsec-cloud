// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::duplicate_mod)]

// The compat module allows to re-use tests from previous major API
#[path = "../v3/pki_enrollment_submit.rs"]
mod compat;

use super::anonymous_cmds;

// Request

pub use compat::req;

// Responses

pub use compat::rep_ok;

pub use compat::rep_already_submitted;

pub use compat::rep_id_already_used;

pub use compat::rep_email_already_used;

pub use compat::rep_already_enrolled;

pub use compat::rep_invalid_payload_data;
