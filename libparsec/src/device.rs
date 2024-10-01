// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

pub use libparsec_platform_device_loader::ArchiveDeviceError;
use libparsec_types::AvailableDevice;

pub async fn list_available_devices(config_dir: &Path) -> Vec<AvailableDevice> {
    libparsec_platform_device_loader::list_available_devices(config_dir).await
}

pub async fn archive_device(device_path: &Path) -> Result<(), ArchiveDeviceError> {
    libparsec_platform_device_loader::archive_device(device_path).await
}
