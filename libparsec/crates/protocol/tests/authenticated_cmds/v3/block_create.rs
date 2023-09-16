// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

// wrap this in each test function
// macro qui contient une closure
use super::authenticated_cmds;

use libparsec_types::prelude::*;

// Request

pub fn req() {
    // TODO #4545: Implement test
}

// Responses

pub fn rep_ok() {
    // TODO #4545: Implement test
}

pub fn rep_already_exists() {
    // TODO #4545: Implement test
}

pub fn rep_not_found() {
    // TODO #4545: Implement test
}

pub fn rep_not_allowed() {
    // TODO #4545: Implement test
}

pub fn rep_in_maintenance() {
    // TODO #4545: Implement test
}

pub fn rep_timeout() {
    // TODO #4545: Implement test
}
