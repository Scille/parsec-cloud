// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native as platform;
#[cfg(target_arch = "wasm32")]
pub(crate) use web as platform;

pub mod certificates;
pub mod user;
pub mod workspace;

// Testbed integration is tested in the `libparsec_tests_fixture` crate.
#[cfg(feature = "test-with-testbed")]
mod testbed;

/// Do not match anything (https://stackoverflow.com/a/2302992/2846140)
pub const PREVENT_SYNC_PATTERN_EMPTY_PATTERN: &str = r"^\b$";
