// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_device_loader::AvailableDevice;
use libparsec_types::prelude::*;

use crate::ClientConfig;

#[derive(Debug, thiserror::Error)]
pub enum RemoveDeviceError {
    #[error("Failed to remove device: {}", .0)]
    DeviceRemovalError(libparsec_platform_device_loader::RemoveDeviceError),
    #[error("Failed to remove device data: {}", .0)]
    DeviceDataRemovalError(anyhow::Error),
}

/// Remove device from existence.
/// That will also remove local associated data to the device (workspaces, certificates, etc).
pub async fn remove_device(
    config: &ClientConfig,
    device: &AvailableDevice,
) -> Result<(), RemoveDeviceError> {
    libparsec_platform_device_loader::remove_device(&config.config_dir, &device.key_file_path)
        .await
        .map_err(RemoveDeviceError::DeviceRemovalError)?;
    libparsec_platform_storage::remove_device_data(&config.data_base_dir, device.device_id)
        .await
        .map_err(RemoveDeviceError::DeviceDataRemovalError)?;
    Ok(())
}
