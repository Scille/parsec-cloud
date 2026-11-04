// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use std::path::Path;

use crate::RenameFileError;

pub async fn rename_file(old: &Path, new: &Path) -> Result<(), RenameFileError> {
    Ok(tokio::fs::rename(old, new).await?)
}
