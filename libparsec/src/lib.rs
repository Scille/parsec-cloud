// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![doc = include_str!("../README.md")]

mod addr;
mod cancel;
mod client;
mod config;
mod device;
mod events;
mod handle;
mod invite;
mod path;
mod platform;
mod recovery;
mod testbed;
mod validation;
mod workspace;
mod workspace_history;
mod workspace_history2;

pub use addr::*;
pub use cancel::*;
pub use client::*;
pub use config::*;
pub use device::{archive_device, list_available_devices, ArchiveDeviceError};
pub use events::*;
pub use handle::Handle;
pub use invite::*;
pub use libparsec_client::{ClientExportRecoveryDeviceError, ImportRecoveryDeviceError};
pub use libparsec_client_connection::*;
pub use libparsec_platform_device_loader::{
    get_default_key_file, is_keyring_available, load_device, save_device, LoadDeviceError,
};
pub use libparsec_platform_storage as storage;
pub use libparsec_protocol::*;
pub use path::*;
pub use platform::*;
pub use recovery::*;
pub use testbed::*;
pub use validation::*;
pub use workspace::*;
pub use workspace_history::*;
pub use workspace_history2::*;

pub mod internal {
    pub use libparsec_client::{
        claimer_retrieve_info, AnyClaimRetrievedInfoCtx, Client, ClientConfig,
        DeviceClaimFinalizeCtx, DeviceClaimInProgress1Ctx, DeviceClaimInProgress2Ctx,
        DeviceClaimInProgress3Ctx, DeviceClaimInitialCtx, DeviceGreetInProgress1Ctx,
        DeviceGreetInProgress2Ctx, DeviceGreetInProgress3Ctx, DeviceGreetInProgress4Ctx,
        DeviceGreetInitialCtx, EventBus, ShamirRecoveryClaimInProgress1Ctx,
        ShamirRecoveryClaimInProgress2Ctx, ShamirRecoveryClaimInProgress3Ctx,
        ShamirRecoveryClaimInitialCtx, ShamirRecoveryClaimMaybeFinalizeCtx,
        ShamirRecoveryClaimMaybeRecoverDeviceCtx, ShamirRecoveryClaimPickRecipientCtx,
        ShamirRecoveryClaimRecoverDeviceCtx, ShamirRecoveryClaimShare, UserClaimFinalizeCtx,
        UserClaimInProgress1Ctx, UserClaimInProgress2Ctx, UserClaimInProgress3Ctx,
        UserClaimInitialCtx, UserClaimListAdministratorsCtx, UserGreetInProgress1Ctx,
        UserGreetInProgress2Ctx, UserGreetInProgress3Ctx, UserGreetInProgress4Ctx,
        UserGreetInitialCtx,
    };
}

/// Libparsec initialization routine
pub async fn init_libparsec(
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] config: ClientConfig,
) {
    // 1) Initialize logger
    #[cfg(not(target_arch = "wasm32"))]
    init_logger(&config);
    log::debug!("Initializing libparsec");

    // 2) Clean base home directory
    #[cfg(not(target_arch = "wasm32"))]
    if let MountpointMountStrategy::Directory { base_dir } = config.mountpoint_mount_strategy {
        if let Err(err) = libparsec_platform_mountpoint::clean_base_mountpoint_dir(base_dir).await {
            log::error!("Failed to clean base home directory ({err})");
        } else {
            log::debug!("Cleaned base home directory");
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
fn init_logger(config: &ClientConfig) {
    let log_env = env_logger::Env::default()
        .filter_or(
            "PARSEC_RUST_LOG",
            config
                .log_level
                .map(|lvl| lvl.to_string())
                .unwrap_or_else(|| "info".to_string()),
        )
        .write_style("PARSEC_RUST_LOG_STYLE");
    let mut builder = env_logger::Builder::from_env(log_env);
    let log_file_path = std::env::var_os("PARSEC_RUST_LOG_FILE")
        .map(Into::into)
        .unwrap_or_else(||
            // TODO: ClientConfig should provide the log directory to use
            // https://github.com/Scille/parsec-cloud/issues/9580
            config.config_dir.join("libparsec.log"));
    let parent = log_file_path.parent();
    if let Err(e) = parent.map(std::fs::create_dir_all).transpose() {
        eprintln!(
            "Failed to create log directory {}: {e}",
            parent.unwrap_or_else(|| std::path::Path::new("")).display()
        );
        eprintln!("The logger will be disabled");
        return;
    }
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&log_file_path)
        .expect("Cannot open log file");
    // FIXME: This is a workaround to be able to get logs from libparsec
    // Since electron seems to block stderr writes from libparsec.
    // But only on unix system, on windows it works fine the logs are display on cmd.
    builder.target(env_logger::Target::Pipe(Box::new(log_file)));

    if let Err(e) = builder.try_init() {
        log::error!("Logging already initialized: {e}")
    } else {
        log::info!("Writing log to {}", log_file_path.display());
    };
}

#[cfg(feature = "cli-tests")]
pub use libparsec_tests_fixtures::{tmp_path, TmpPath};
