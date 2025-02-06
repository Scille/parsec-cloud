// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Reexport 3rd parties needed by `parsec_test` macro

pub use log;

// In theory we'd like to expose rstest internal `#[rstest]` and `#[fixture]`
// attributes, however we must instead expose the module with it original name
// (hence shadowing the `#[rstest]` attribute).
// This is due to how some of rstest features are implemented (e.g. magic conversions).
pub use rstest;

// Platform modules re-expose specific dependencies that are only available
// on a single platform, and are used by the `#[parsec_test]` attribute macro.
//
// This way when crate X wants to use the `#[parsec_test]` attribute macro, it
// only has to depend on `libparsec_tests_lite` which always contains the needed
// dependencies (instead of having to manually declare them in crate X).

#[cfg(not(target_arch = "wasm32"))]
pub mod platform {
    pub use env_logger;
    pub use tokio;
}

#[cfg(target_arch = "wasm32")]
pub mod platform {
    pub use console_error_panic_hook;
    pub use console_log;
    // Note `rstest` requires the tester attribute to be called `test`
    // (see https://github.com/la10736/rstest/issues/211)
    pub use wasm_bindgen_test::wasm_bindgen_test as test;
    pub use wasm_bindgen_test::wasm_bindgen_test_configure;
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

#[cfg(feature = "parsec_test_macro")]
pub use libparsec_tests_macros::parsec_test;

pub mod prelude {
    pub use super::*;
    // Here we can expose the actual stuff we care in rstest !
    pub use rstest::{fixture, rstest};

    pub const USER_CERTIF: [u8; 265] = hex!(
        "fee16e1a7aff6fbe84eb9f1b3008e37cc8ca7cb27b4fd49b8c787085d2451fd09e4e1c"
        "f471f23c916a5a8524ff27dfa1ce3de3568949c83140ecd9250d2f0702789c01be0041"
        "ff88a474797065b0757365725f6365727469666963617465a6617574686f72aa616c69"
        "63654064657631a974696d657374616d70d70141d782f840000000a7757365725f6964"
        "a3626f62ac68756d616e5f68616e646c6592af626f62406578616d706c652e636f6dae"
        "426f6279204d63426f6246616365aa7075626c69635f6b6579c4207c999e9980bef377"
        "07068b07d975591efc56335be9634ceef7c932a09c891e25a869735f61646d696ec2a7"
        "70726f66696c65a85354414e44415244a7225235"
    );

    pub const REDACTED_USER_CERTIF: [u8; 234] = hex!(
        "e04ff4746d51000f68eb3dcfde36c53fe60f919c5eb8e76bbe2e4748b71553484cf5e4"
        "a73fde1650736538d5f70f83205307b216d3109ca1b9ca66f453089009789c019f0060"
        "ff88a474797065b0757365725f6365727469666963617465a6617574686f72aa616c69"
        "63654064657631a974696d657374616d70d70141d782f840000000a7757365725f6964"
        "a3626f62ac68756d616e5f68616e646c65c0aa7075626c69635f6b6579c4207c999e99"
        "80bef37707068b07d975591efc56335be9634ceef7c932a09c891e25a869735f61646d"
        "696ec2a770726f66696c65a85354414e44415244fe7e465c"
    );

    pub const DEVICE_CERTIF: [u8; 231] = hex!(
        "1e600d29590d2e005e19cdc7b2a75b54a549c15e9eb57213aa5bdd3696d0a7c68cb88f"
        "3209d1e682cfa5754229165924900a5e56ef571dc53fa2ba4ba51e0c04789c019c0063"
        "ff86a474797065b26465766963655f6365727469666963617465a6617574686f72aa61"
        "6c6963654064657631a974696d657374616d70d70141d782f840000000a96465766963"
        "655f6964a8626f624064657631ac6465766963655f6c6162656caf4d79206465763120"
        "6d616368696e65aa7665726966795f6b6579c420840d872f4252da2d1c9f81a77db5f0"
        "a5b9b60a5cde1eeabf40388ef6bca649094f5b436f"
    );

    pub const REDACTED_DEVICE_CERTIF: [u8; 216] = hex!(
        "2e44c56592b613a94ae151cc3b754945e300f457eb71d9f6af18f026f943bd0c1b3a02"
        "9c1fbc504aa793f7937067864cb4fc0c7eeba7e1f9d83dfec5fb9b3d01789c018d0072"
        "ff86a474797065b26465766963655f6365727469666963617465a6617574686f72aa61"
        "6c6963654064657631a974696d657374616d70d70141d782f840000000a96465766963"
        "655f6964a8626f624064657631ac6465766963655f6c6162656cc0aa7665726966795f"
        "6b6579c420840d872f4252da2d1c9f81a77db5f0a5b9b60a5cde1eeabf40388ef6bca6"
        "4909da083e35"
    );
}
