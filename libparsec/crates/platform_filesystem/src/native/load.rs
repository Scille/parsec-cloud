// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::LoadFileError;
use libparsec_types::Bytes;
use std::path::Path;

pub async fn load_file(path: &Path) -> Result<Bytes, LoadFileError> {
    if !path.exists() {
        // Because if file does not exists it's not a file
        return Err(LoadFileError::NotFound);
    }
    if !path.is_file() {
        // Because when windows attempts to read a directory it returns an invalid path error
        return Err(LoadFileError::NotAFile);
    }
    Ok(Bytes::from(tokio::fs::read(path).await?))
}
