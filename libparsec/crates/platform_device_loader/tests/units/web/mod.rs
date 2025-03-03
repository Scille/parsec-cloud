#![cfg(target_arch = "wasm32")]
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod storage;

libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser);
