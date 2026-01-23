// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::common::internal::Storage;
use super::common::wrapper::OpenOptions;
use crate::SaveContentError;

pub async fn save_content(path: &std::path::Path, content: &[u8]) -> Result<(), SaveContentError> {
    let Ok(store) = Storage::new().await.inspect_err(|e| {
        log::error!("Failed to access storage: {e}");
    }) else {
        return Err(SaveContentError::StorageNotAvailable);
    };
    log::trace!("Saving device file at {}", path.display());
    let parent = if let Some(parent) = path.parent() {
        Some(store.root_dir().create_dir_all(parent).await?)
    } else {
        None
    };
    let file = parent
        .as_ref()
        .unwrap_or(&store.root_dir())
        .get_file(
            path.file_name()
                .and_then(std::ffi::OsStr::to_str)
                .ok_or(SaveContentError::InvalidPath)?,
            Some(OpenOptions::create()),
        )
        .await?;
    file.write_all(&content).await.map_err(Into::into)
}
