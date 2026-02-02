// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_filesystem::{remove_file, RemoveFileError};

use std::path::Path;

use libparsec_types::prelude::*;

#[cfg(feature = "test-with-testbed")]
use crate::testbed;

#[derive(Debug, thiserror::Error)]
pub enum RemoveDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error("Device not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<RemoveFileError> for RemoveDeviceError {
    fn from(value: RemoveFileError) -> Self {
        match value {
            RemoveFileError::NotFound => RemoveDeviceError::NotFound,
            RemoveFileError::StorageNotAvailable => RemoveDeviceError::StorageNotAvailable,
            RemoveFileError::InvalidParent
            | RemoveFileError::InvalidPath
            | RemoveFileError::Internal(_) => RemoveDeviceError::Internal(value.into()),
        }
    }
}

pub async fn remove_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    device_path: &Path,
) -> Result<(), RemoveDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_remove_device(config_dir, device_path) {
        return result;
    }

    Ok(remove_file(device_path).await?)
}
