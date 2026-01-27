// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::common::internal::Storage;
use crate::RemoveFileError;
use std::path::Path;

pub async fn remove_file(path: &Path) -> Result<(), RemoveFileError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(RemoveFileError::StorageNotAvailable);
    };
    Ok(storage.root_dir().remove_entry_from_path(path).await?)
}
