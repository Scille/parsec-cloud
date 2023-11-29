// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use neon::prelude::*;
use once_cell::sync::OnceCell;
use sentry::{ClientInitGuard, ClientOptions};
use std::sync::Mutex;

mod meths;

lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: Mutex<tokio::runtime::Runtime> =
        Mutex::new(tokio::runtime::Runtime::new().expect("Cannot start tokio runtime for libparsec"));
}

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

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    meths::register_meths(&mut cx)
}
