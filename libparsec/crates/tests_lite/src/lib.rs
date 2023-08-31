// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Reexport 3rd parties needed by `parsec_test` macro
pub use env_logger;
pub use rstest;
#[cfg(not(target_arch = "wasm32"))]
pub use tokio;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_test;
// Pretty assertions are super useful in tests, the only issue is we cannot use star-use
// to import them (given they shadow the default asserts), so we expose them with a
// sightly different name
pub use pretty_assertions::{
    assert_eq as p_assert_eq, assert_matches as p_assert_matches, assert_ne as p_assert_ne,
    assert_str_eq as p_assert_str_eq,
};

pub use libparsec_tests_macros::parsec_test;

#[cfg(target_arch = "wasm32")]
// FIXME: Remove me once https://github.com/la10736/rstest/issues/211 is resolved
pub mod wasm {
    pub use wasm_bindgen_test::wasm_bindgen_test as test;
}

pub mod prelude {
    pub use super::*;
}
