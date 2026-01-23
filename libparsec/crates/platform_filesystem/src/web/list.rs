// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use super::common::internal::Storage;
use crate::ListFilesError;

pub async fn list_files(
    root_dir: &Path,
    extension: impl AsRef<OsStr> + std::fmt::Display,
) -> Result<Vec<PathBuf>, ListFilesError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(ListFilesError::StorageNotAvailable);
    };
    Ok(storage.list_file_entries(root_dir, extension).await?)
}
