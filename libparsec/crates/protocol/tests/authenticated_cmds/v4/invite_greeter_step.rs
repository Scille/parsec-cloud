// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::{InvitationStatus, InvitationToken};

// Request

pub fn req() {
    todo!()
}

pub fn rep_ok() {
    todo!()
}

pub fn rep_not_ready() {
    todo!()
}

pub fn rep_invitation_not_found() {
    todo!()
}

pub fn rep_invitation_completed() {
    todo!()
}

pub fn rep_invitation_cancelled() {
    todo!()
}

pub fn rep_author_not_allowed() {
    todo!()
}

pub fn rep_attempt_not_found() {
    todo!()
}

pub fn rep_attempt_not_joined() {
    todo!()
}

pub fn rep_attempt_cancelled() {
    todo!()
}

pub fn rep_step_too_advanced() {
    todo!()
}

pub fn rep_step_mismatch() {
    todo!()
}
