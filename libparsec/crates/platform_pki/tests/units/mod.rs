// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_arch = "wasm32")]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser run_in_shared_worker);

mod encrypt;
#[cfg(target_os = "windows")] // TODO: libparsec_platform_pki only supports Windows so far
mod list;
#[cfg(not(target_os = "windows"))]
mod scws;
mod shared;
#[cfg(target_os = "windows")] // TODO: libparsec_platform_pki only supports Windows so far
mod sign;
mod utils;
mod x509;
