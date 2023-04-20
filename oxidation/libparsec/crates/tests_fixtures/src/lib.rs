// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub use libparsec_tests_macros::parsec_test;

// Must rename `tmp_path` module to provent it `tmp_path` item from being shadowed
#[path = "./tmp_path.rs"]
mod tmp_path_mod;
mod trustchain;

pub use tmp_path_mod::*;
pub use trustchain::*;

// Reexport 3rd parties needed by `parsec_test` macro
pub use env_logger;
pub use rstest;

// Reexport so that `use libparsec_tests_fixtures::prelude::*` is the single-no-brainer-one-liner™
pub use libparsec_testbed::*;
pub use libparsec_types::fixtures::*;

pub mod prelude {
    pub use super::*;
}
