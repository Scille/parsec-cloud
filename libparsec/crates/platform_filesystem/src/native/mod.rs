// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_types::anyhow;

use crate::SaveContentError;

pub async fn save_content(key_file: &PathBuf, file_content: &[u8]) -> Result<(), SaveContentError> {
    if let Some(parent) = key_file.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| SaveContentError::InvalidPath(e.into()))?;
    }
    let tmp_path = match key_file.file_name() {
        Some(file_name) => {
            let mut tmp_path = key_file.clone();
            {
                let mut tmp_file_name = file_name.to_owned();
                tmp_file_name.push(".tmp");
                tmp_path.set_file_name(tmp_file_name);
            }
            tmp_path
        }
        None => {
            return Err(SaveContentError::InvalidPath(anyhow::anyhow!(
                "Path is missing a file name"
            )))
        }
    };

    // Classic pattern for atomic file creation:
    // - First write the file in a temporary location
    // - Then move the file to it final location
    // This way a crash during file write won't end up with a corrupted
    // file in the final location.
    tokio::fs::write(&tmp_path, file_content)
        .await
        .map_err(|e| SaveContentError::InvalidPath(e.into()))?;
    tokio::fs::rename(&tmp_path, key_file)
        .await
        .map_err(|e| SaveContentError::InvalidPath(e.into()))?;

    Ok(())
}
