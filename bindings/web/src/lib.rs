// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![cfg(target_arch = "wasm32")]

use js_sys::JsString;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

mod meths;
pub use meths::*;

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn initLogger(default_log_level: JsString) {
    #[cfg(feature = "console_error_panic_hook")]
    {
        console_error_panic_hook::set_once();
    }
    const DEFAULT_LOG_LEVEL: log::Level = log::Level::Info;
    let default_log_level = if default_log_level.is_undefined() {
        DEFAULT_LOG_LEVEL
    } else {
        match String::from(&default_log_level).to_lowercase().as_str() {
            "trace" => log::Level::Trace,
            "debug" => log::Level::Debug,
            "info" => log::Level::Info,
            "warn" => log::Level::Warn,
            "error" => log::Level::Error,
            _ => DEFAULT_LOG_LEVEL,
        }
    };
    console_log::init_with_level(default_log_level).expect("cannot initialize console logger");
}
