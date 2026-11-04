// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use crate::RemoveFileError;
use std::path::Path;

pub async fn remove_file(path: &Path) -> Result<(), RemoveFileError> {
    tokio::fs::remove_file(path).await.map_err(Into::into)
}
