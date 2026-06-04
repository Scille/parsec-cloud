// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

//use libparsec_tests_lite::prelude::*;

//use super::authenticated_cmds;

// Request

pub fn req() {}

// Responses

pub fn rep_ok() {}

pub fn rep_workspace_not_found() {}

pub fn rep_file_not_found() {}
