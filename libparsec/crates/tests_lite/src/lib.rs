// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Reexport 3rd parties needed by `parsec_test` macro

pub use env_logger;
// In theory we'd like to expose rstest internal `#[rstest]` and `#[fixture]`
// attributes, however we must instead expose the module with it original name
// (hence shadowing the `#[rstest]` attribute).
// This is due to how some of rstest features are implemented (e.g. magic conversions).
pub use rstest;
#[cfg(not(target_arch = "wasm32"))]
pub use tokio;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_test;
#[cfg(target_arch = "wasm32")]
// FIXME: Remove me once https://github.com/la10736/rstest/issues/211 is resolved
pub mod wasm {
    pub use wasm_bindgen_test::wasm_bindgen_test as test;
}

// Reexport 3rd parties useful for testing

pub use hex_literal::hex;

// Pretty assertions are super useful in tests, the only issue is we cannot use star-use
// to import them (given they shadow the default asserts), so we expose them with a
// sightly different name
pub use pretty_assertions::{
    assert_eq as p_assert_eq, assert_matches as p_assert_matches, assert_ne as p_assert_ne,
    assert_str_eq as p_assert_str_eq,
};

// Export our own stuff

pub use libparsec_tests_macros::parsec_test;

pub mod prelude {
    pub use super::*;
    // Here we can expose the actual stuff we care in rstest !
    pub use rstest::{fixture, rstest};
}
