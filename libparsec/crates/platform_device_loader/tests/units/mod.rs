// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_arch = "wasm32")]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser);

mod archive;
mod change_auth;
mod list;
mod load;
mod recovery;
mod remove;
mod save;
mod save_load;
mod utils;

use libparsec_tests_lite::p_assert_eq;
use libparsec_types::LocalDevice;

#[test]
fn test_server_url_from_device() {
    let device = LocalDevice::generate_new_device(
        "parsec3://parsec.example.com/MyOrg?p=xCBs8zpdIwovR8EdliVVo2vUOmtumnfsI6Fdndjm0WconA" // cspell:disable-line
            .parse()
            .unwrap(),
        libparsec_types::UserProfile::Standard,
        "Alice <alice@example.com>".parse().unwrap(),
        "alice@dev1".parse().unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    let url = super::server_url_from_device(&device);
    p_assert_eq!(url, "https://parsec.example.com/");
}
