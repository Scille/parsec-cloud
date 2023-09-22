// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::duplicate_mod)]

use super::authenticated_cmds;

#[path = "../v3/invite_list.rs"]
mod compat;

// Request

pub use compat::req;

// Responses

pub use compat::rep_ok;
