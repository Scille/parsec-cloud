// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![cfg(target_arch = "wasm32")]

#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

mod meths;
pub use meths::*;

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn initLogger() {
    #[cfg(feature = "console_error_panic_hook")]
    {
        console_error_panic_hook::set_once();
    }
    let log_level = if cfg!(debug_assertions) {
        log::Level::Trace
    } else {
        log::Level::Info
    };
    console_log::init_with_level(log_level).expect("cannot initialize console logger");
    log::info!("Libparsec logger initialized with level {log_level}");
}
