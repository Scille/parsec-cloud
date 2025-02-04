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
