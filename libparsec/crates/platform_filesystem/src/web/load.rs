// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::common::internal::Storage;
use crate::LoadFileError;
use libparsec_types::Bytes;
use std::path::Path;

pub async fn load_file(path: &Path) -> Result<Bytes, LoadFileError> {
    let Ok(storage) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(LoadFileError::StorageNotAvailable);
    };
    Ok(Bytes::from(storage.read_file(path).await?))
}
