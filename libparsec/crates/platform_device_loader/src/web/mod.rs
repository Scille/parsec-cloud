// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod error;
mod internal;

use std::{path::Path, sync::Arc};

use libparsec_types::prelude::{AvailableDevice, DateTime, DeviceAccessStrategy, LocalDevice};

use internal::Storage;

/*
 * List available devices
 */

pub async fn list_available_devices(config_dir: &Path) -> Vec<AvailableDevice> {
    let Ok(storage) = Storage::new().inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Vec::new();
    };
    storage
        .list_available_devices(config_dir)
        .inspect(|v| {
            log::trace!("Found the following devices: {v:?}");
        })
        .inspect_err(|e| {
            log::error!("Failed to list available devices: {e}");
        })
        .unwrap_or_default()
}

/*
 * Save & load
 */

pub async fn load_device(
    access: &DeviceAccessStrategy,
) -> Result<(Arc<LocalDevice>, DateTime), crate::LoadDeviceError> {
    let storage = Storage::new().inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    })?;
    storage.load_device(access).map_err(Into::into)
}

pub async fn save_device(
    access: &DeviceAccessStrategy,
    device: &LocalDevice,
    created_on: DateTime,
) -> Result<AvailableDevice, crate::SaveDeviceError> {
    let storage = Storage::new().inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    })?;
    storage
        .save_device(access, device, created_on)
        .map_err(Into::into)
}

pub async fn change_authentication(
    current_access: &DeviceAccessStrategy,
    new_access: &DeviceAccessStrategy,
) -> Result<AvailableDevice, crate::ChangeAuthenticationError> {
    let storage = Storage::new().inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    })?;
    let (device, created_on) = storage.load_device(current_access)?;
    let available_device = storage.save_device(new_access, &device, created_on)?;

    let key_file = current_access.key_file();
    let new_key_file = new_access.key_file();

    if key_file != new_key_file {
        storage.remove_device(key_file)?;
    }

    Ok(available_device)
}

pub async fn archive_device(device_path: &Path) -> Result<(), crate::ArchiveDeviceError> {
    let storage = Storage::new().inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    })?;
    storage.archive_device(device_path).map_err(Into::into)
}

pub async fn remove_device(device_path: &Path) -> Result<(), crate::RemoveDeviceError> {
    let storage = Storage::new().inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    })?;
    storage.remove_device(device_path).map_err(Into::into)
}
