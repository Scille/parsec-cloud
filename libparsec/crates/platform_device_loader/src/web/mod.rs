// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub(crate) mod error;
pub(crate) mod internal;
pub(crate) mod wrapper;

use std::{path::Path, sync::Arc};

use libparsec_types::prelude::*;

use crate::{
    ArchiveDeviceError, ListAvailableDeviceError, LoadDeviceError, RemoveDeviceError,
    SaveDeviceError, UpdateDeviceError,
};
use internal::Storage;

/*
 * List available devices
 */

pub async fn list_available_devices(
    config_dir: &Path,
) -> Result<Vec<AvailableDevice>, ListAvailableDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(ListAvailableDeviceError::StorageNotAvailable);
    };
    storage
        .list_available_devices(config_dir)
        .await
        .inspect(|v| {
            log::trace!("Found the following devices: {v:?}");
        })
        .inspect_err(|e| {
            log::error!("Failed to list available devices: {e}");
        })
        .map_err(|e| ListAvailableDeviceError::Internal(anyhow::anyhow!("{e}")))
}

/*
 * Save & load
 */

pub async fn load_device(
    access: &DeviceAccessStrategy,
) -> Result<(Arc<LocalDevice>, DateTime), LoadDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(LoadDeviceError::StorageNotAvailable);
    };
    storage.load_device(access).await.map_err(Into::into)
}

pub async fn save_device(
    access: &DeviceAccessStrategy,
    device: &LocalDevice,
    created_on: DateTime,
) -> Result<AvailableDevice, SaveDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(SaveDeviceError::StorageNotAvailable);
    };
    storage
        .save_device(access, device, created_on)
        .await
        .map_err(Into::into)
}

pub async fn update_device(
    current_access: &DeviceAccessStrategy,
    new_access: &DeviceAccessStrategy,
    overwrite_server_addr: Option<ParsecAddr>,
) -> Result<(AvailableDevice, ParsecAddr), UpdateDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(UpdateDeviceError::StorageNotAvailable);
    };

    let (mut device, created_on) = storage.load_device(current_access).await?;

    let old_server_addr = ParsecAddr::new(
        device.organization_addr.hostname().to_owned(),
        Some(device.organization_addr.port()),
        device.organization_addr.use_ssl(),
    );
    if let Some(overwrite_server_addr) = overwrite_server_addr {
        Arc::make_mut(&mut device).organization_addr = ParsecOrganizationAddr::new(
            overwrite_server_addr,
            device.organization_addr.organization_id().to_owned(),
            device.organization_addr.root_verify_key().to_owned(),
        );
    }

    let available_device = storage.save_device(new_access, &device, created_on).await?;

    let key_file = current_access.key_file();
    let new_key_file = new_access.key_file();

    if key_file != new_key_file {
        if let Err(err) = storage.remove_device(key_file).await {
            log::warn!("Cannot remove old key file {key_file:?}: {err}");
        }
    }

    Ok((available_device, old_server_addr))
}

pub async fn archive_device(device_path: &Path) -> Result<(), ArchiveDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(ArchiveDeviceError::StorageNotAvailable);
    };
    storage
        .archive_device(device_path)
        .await
        .map_err(Into::into)
}

pub async fn remove_device(device_path: &Path) -> Result<(), RemoveDeviceError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(RemoveDeviceError::StorageNotAvailable);
    };
    storage.remove_device(device_path).await.map_err(Into::into)
}
