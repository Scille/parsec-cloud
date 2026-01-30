// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use itertools::Itertools;
use libparsec_platform_filesystem::{list_files, ListFilesError};
use std::path::Path;

use libparsec_types::prelude::*;

#[cfg(feature = "test-with-testbed")]
use crate::testbed;
use crate::{get_devices_dir, load_available_device, AvailableDevice, DEVICE_FILE_EXT};

#[derive(Debug, thiserror::Error)]
pub enum ListAvailableDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<ListFilesError> for ListAvailableDeviceError {
    fn from(value: ListFilesError) -> Self {
        match value {
            ListFilesError::StorageNotAvailable => ListAvailableDeviceError::StorageNotAvailable,
            ListFilesError::InvalidParent | ListFilesError::Internal(_) => {
                ListAvailableDeviceError::Internal(value.into())
            }
        }
    }
}

/// On web `config_dir` is used as database discriminant when using IndexedDB API
pub async fn list_available_devices(
    config_dir: &Path,
) -> Result<Vec<AvailableDevice>, ListAvailableDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_list_available_devices(config_dir) {
        return Ok(result);
    }

    let devices_dir = get_devices_dir(config_dir);

    // Consider `.keys` files in devices directory
    let mut key_file_paths = list_files(&devices_dir, DEVICE_FILE_EXT).await?;

    // Sort paths so the discovery order is deterministic
    // In the case of duplicate files, that means only the first discovered device is considered
    key_file_paths.sort();
    log::trace!("Found the following device files: {key_file_paths:?}");

    let mut res = Vec::with_capacity(key_file_paths.len());
    for key_file in key_file_paths {
        if let Ok(device) = load_available_device(config_dir, key_file.clone())
            .await
            .inspect_err(|e| {
                log::debug!(
                    "Failed to load device at {path} with {e}",
                    path = key_file.display()
                )
            })
        {
            res.push(device);
        }
    }
    Ok(res.into_iter().unique_by(|v| v.device_id).collect_vec())
}
