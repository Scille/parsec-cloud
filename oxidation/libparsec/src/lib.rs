// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub use libparsec_client_high_level_api::*;

#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_client_types as client_types;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_core_fs as core_fs;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_protocol as protocol;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_types as types;

pub use libparsec_core as core;
pub use libparsec_crypto as crypto;
pub use libparsec_platform_device_loader as platform_device_loader;

// TODO: replace me by the high-level API here ;-)
pub use libparsec_client_types::{AvailableDevice, DeviceFileType, StrPath};
pub use libparsec_core::{
    logged_core_get_device_display, logged_core_get_device_id, login, LoggedCoreError,
    LoggedCoreHandle, LoggedCoreResult,
};

pub async fn list_available_devices(config_dir: StrPath) -> Vec<AvailableDevice> {
    libparsec_platform_device_loader::list_available_devices(&std::path::PathBuf::from(config_dir))
        .await
}
