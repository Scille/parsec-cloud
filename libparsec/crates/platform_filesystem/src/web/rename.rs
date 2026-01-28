// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::platform::common::internal::Storage;
use crate::RenameFileError;
use std::path::Path;

pub async fn rename_file(old: &Path, new: &Path) -> Result<(), RenameFileError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(RenameFileError::StorageNotAvailable);
    };
    storage.rename_file(old, new).await
}
