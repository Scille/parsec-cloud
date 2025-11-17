// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_arch = "wasm32")]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser run_in_shared_worker);

mod archive;
mod list;
mod load;
mod pki;
mod recovery;
mod remove;
mod save;
mod save_list;
mod save_load;
mod strategy;
mod update_device_change_authentication;
mod update_device_overwrite_server_addr;
mod utils;
