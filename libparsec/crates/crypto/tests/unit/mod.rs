// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod anti_leak;
mod bad_keygen;
mod hash;
mod key_derivation;
mod password;
mod private;
mod sas;
mod secret;
mod sequester;
mod sign;
mod utils;

#[cfg(target_arch = "wasm32")]
mod platform {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser run_in_shared_worker);
    pub use wasm_bindgen_test::wasm_bindgen_test as test;
}
#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub use test;
}
