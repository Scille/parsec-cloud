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

#[cfg(feature = "cli-tests")]
pub use libparsec_tests_fixtures::{tmp_path, TmpPath};
