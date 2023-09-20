// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::duplicate_mod)]

use super::authenticated_cmds;

// The compat module allows to re-use tests from previous major API
#[path = "../v3/ping.rs"]
mod compat;

// Request

pub use compat::req;

// Responses

pub use compat::rep_ok;
