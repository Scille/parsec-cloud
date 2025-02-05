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

// TODO: Read from env var
static SENTRY_DSN_LIBPARSEC: &str =
    "https://215e9cba2353323a1c5156cf8f845129@o155936.ingest.us.sentry.io/4507644297019392";
static SENTRY_CLIENT_GUARD: OnceCell<ClientInitGuard> = OnceCell::new();

fn init_sentry() {
    SENTRY_CLIENT_GUARD.get_or_init(|| {
        sentry::init((
            SENTRY_DSN_LIBPARSEC,
            ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ))
    });
}

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    meths::register_meths(&mut cx)
}
