// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod oneshot;
mod task;

#[cfg(target_arch = "wasm32")]
libparsec_tests_lite::wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
