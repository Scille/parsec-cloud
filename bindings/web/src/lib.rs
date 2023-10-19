// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![cfg(target_arch = "wasm32")]

use once_cell::sync::OnceCell;
use sentry::{ClientInitGuard, ClientOptions};
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

mod meths;
pub use meths::*;

static SENTRY: OnceCell<ClientInitGuard> = OnceCell::new();

fn init_sentry() {
    if let Ok(sentry_url) = std::env::var("SENTRY_URL") {
        SENTRY.get_or_init(|| {
            sentry::init((
                sentry_url,
                ClientOptions {
                    release: sentry::release_name!(),
                    ..Default::default()
                },
            ))
        });
    }
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn initLogger() {
    #[cfg(feature = "console_error_panic_hook")]
    {
        console_error_panic_hook::set_once();
    }
    console_log::init_with_level(log::Level::Info).expect("cannot initialize console logger");
}
