use std::path::PathBuf;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub const PARSEC_RUST_LOG: &str = "PARSEC_RUST_LOG";
pub const PARSEC_RUST_LOG_FILE: &str = "PARSEC_RUST_LOG_FILE";

#[derive(Debug)]
pub struct LogConfig {
    pub level: Option<LevelFilter>,
    pub log_file: Option<PathBuf>,
    pub log_console: bool,
}

pub fn init_logger(config: LogConfig) -> anyhow::Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(config.level.unwrap_or(LevelFilter::INFO).into())
        .with_env_var(PARSEC_RUST_LOG)
        // Ignore invalid directive
        .from_env_lossy();

    #[cfg(not(target_family = "wasm"))]
    let pretty_console = tracing_subscriber::fmt::layer().pretty();
    #[cfg(target_family = "wasm")]
    let pretty_console = tracing_subscriber::fmt::layer()
        .json()
        // Disable color
        .with_ansi(false)
        // For some reason, if we don't do this in the browser, we get
        // a runtime error.
        .without_time()
        .with_writer(tracing_subscriber_wasm::MakeConsoleWriter::new());

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(if config.log_console {
            Some(pretty_console)
        } else {
            None
        })
        .with(if cfg!(target_family = "wasm") {
            None
        } else {
            // We give priority to env var over config value
            std::env::var_os(PARSEC_RUST_LOG_FILE)
                .map(PathBuf::from)
                .as_ref()
                .or(config.log_file.as_ref())
                .map(|path| -> anyhow::Result<_> {
                    use anyhow::Context;
                    path.parent()
                        .map(std::fs::create_dir_all)
                        .transpose()
                        .context("Failed to create parent folder for log file")?;
                    let log_file = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(path)
                        .context("Failed to open log file")?;

                    Ok(tracing_subscriber::fmt::layer()
                        .json()
                        .with_writer(log_file))
                })
                .transpose()?
        });

    if let Err(e) = subscriber.try_init() {
        tracing::error!(err = %e, "Logging already initialized");
    } else {
        tracing::info!(?config, "Logger configured");
    };

    Ok(())
}

#[cfg(feature = "example")]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn test_tracing() {
    // Test tracing
    tracing::trace!("Trace level trace");
    tracing::debug!("Debug level trace");
    tracing::info!("Info level trace");
    tracing::warn!("Warn level trace");
    tracing::error!("Error level trace");

    // Test log
    log::trace!("Trace level log");
    log::debug!("Debug level log");
    log::info!("Info level log");
    log::warn!("Warn level log");
    log::error!("Error level log");

    tracing::error!(
        err = "I'm the error msg",
        "We've got an error log by tracing"
    );
    log::error!(err = "I'm the error msg"; "We've got an error log by log");
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn web_init_logger() {
    let config = LogConfig {
        level: Some(LevelFilter::TRACE),
        log_file: None,
        log_console: true,
    };
    init_logger(config).unwrap();
}
