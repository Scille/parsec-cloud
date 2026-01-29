// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::RemoveFileError;
use std::path::Path;

pub async fn remove_file(path: &Path) -> Result<(), RemoveFileError> {
    tokio::fs::remove_file(path).await.map_err(Into::into)
}
