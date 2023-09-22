// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::duplicate_mod)]

// The compat module allows to re-use tests from previous major API
#[path = "../v3/invite_info.rs"]
mod compat;

use super::invited_cmds;

// Request

pub use compat::req;

// Responses

pub use compat::rep_ok;
