// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;

use crate::RenameFileError;

pub async fn rename_file(old: &Path, new: &Path) -> Result<(), RenameFileError> {
    Ok(tokio::fs::rename(old, new).await?)
}
