// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_filesystem::{rename_file, RenameFileError};

use std::path::{Path, PathBuf};

use libparsec_types::prelude::*;

#[cfg(feature = "test-with-testbed")]
use crate::testbed;
use crate::ARCHIVE_DEVICE_EXT;

#[derive(Debug, thiserror::Error)]
pub enum ArchiveDeviceError {
    #[error("No space available")]
    NoSpaceAvailable,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(crate) fn get_device_archive_path(path: &Path) -> PathBuf {
    if let Some(current_file_extension) = path.extension() {
        // Add ARCHIVE_DEVICE_EXT to the current file extension resulting in extension `.{current}.{ARCHIVE_DEVICE_EXT}`.
        let mut ext = current_file_extension.to_owned();
        ext.extend([".".as_ref(), ARCHIVE_DEVICE_EXT.as_ref()]);
        path.with_extension(ext)
    } else {
        path.with_extension(ARCHIVE_DEVICE_EXT)
    }
}

/// Archive a device identified by its path.
pub async fn archive_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    device_path: &Path,
) -> Result<(), ArchiveDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_archive_device(config_dir, device_path) {
        return result;
    }

    let archive_device_path = get_device_archive_path(device_path);

    log::debug!(
        "Archiving device {} to {}",
        device_path.display(),
        archive_device_path.display()
    );

    rename_file(device_path, &archive_device_path)
        .await
        .map_err(|e| match e {
            RenameFileError::StorageNotAvailable | RenameFileError::NoSpaceLeft => {
                ArchiveDeviceError::NoSpaceAvailable
            }
            RenameFileError::InvalidParent
            | RenameFileError::InvalidPath
            | RenameFileError::NotFound
            | RenameFileError::Internal(_) => ArchiveDeviceError::Internal(e.into()),
        })
}
