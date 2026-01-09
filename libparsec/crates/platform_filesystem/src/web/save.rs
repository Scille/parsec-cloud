// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::web::common::internal::Storage;
use crate::web::common::wrapper::OpenOptions;
use crate::SaveContentError;

pub async fn save_content(
    store: Storage,
    path: &std::path::Path,
    content: &[u8],
) -> Result<(), SaveContentError> {
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
