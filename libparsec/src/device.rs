// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

pub use libparsec_platform_device_loader::{
    ArchiveDeviceError, ListAvailableDeviceError, UpdateDeviceError,
};
use libparsec_types::prelude::*;
pub use libparsec_types::AvailableDevice;

pub async fn list_available_devices(
    config_dir: &Path,
) -> Result<Vec<AvailableDevice>, ListAvailableDeviceError> {
    libparsec_platform_device_loader::list_available_devices(config_dir).await
}

pub async fn archive_device(device_path: &Path) -> Result<(), ArchiveDeviceError> {
    libparsec_platform_device_loader::archive_device(device_path).await
}

pub async fn update_device_change_authentication(
    config_dir: &Path,
    current_auth: DeviceAccessStrategy,
    new_auth: DeviceSaveStrategy,
) -> Result<AvailableDevice, UpdateDeviceError> {
    let key_file = current_auth.key_file().to_owned();

    libparsec_platform_device_loader::update_device_change_authentication(
        config_dir,
        &current_auth,
        &new_auth.into_access(key_file),
    )
    .await
}

pub async fn update_device_overwrite_server_addr(
    config_dir: &Path,
    auth: DeviceAccessStrategy,
    new_server_addr: ParsecAddr,
) -> Result<ParsecAddr, UpdateDeviceError> {
    libparsec_platform_device_loader::update_device_overwrite_server_addr(
        config_dir,
        &auth,
        new_server_addr,
    )
    .await
}
