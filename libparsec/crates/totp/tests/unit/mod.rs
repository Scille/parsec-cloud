// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod totp_create_opaque_key;
mod totp_fetch_opaque_key;
mod totp_setup_confirm_anonymous;
mod totp_setup_confirm_authenticated;
mod totp_setup_status_anonymous;
mod totp_setup_status_authenticated;
mod utils;

#[cfg(target_arch = "wasm32")]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser run_in_shared_worker);
