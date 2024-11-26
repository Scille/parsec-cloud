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
    let env = env_logger::Env::default()
        .default_filter_or("libparsec_platform_mountpoint=trace,fuser=trace");
    let mut builder = env_logger::Builder::from_env(env);
    // FIXME: This is a workaround to be able to get logs from libparsec
    // Since electron seems to block stderr writes from libparsec.
    // But only on unix system, on windows it works fine the logs are display on cmd.
    #[cfg(target_family = "unix")]
    let log_file_path = {
        let now = libparsec::DateTime::now();
        let log_filename = format!("libparsec-{}.log", now.to_rfc3339());
        let log_file_path = std::env::temp_dir().join(log_filename);
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_file_path)
            .expect("Cannot open log file");
        builder.target(env_logger::Target::Pipe(Box::new(log_file)));
        log_file_path
    };

    if let Err(e) = builder.try_init() {
        log::error!("Logging already initialized: {e}")
    } else {
        #[cfg(target_family = "unix")]
        log::info!("Writing log to {}", log_file_path.display());
    };

    meths::register_meths(&mut cx)
}
