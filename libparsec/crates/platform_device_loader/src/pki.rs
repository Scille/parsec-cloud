// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_async::stream::StreamExt;
use libparsec_platform_filesystem::{
    list_files, load_file, save_content, ListFilesError, LoadFileError, SaveContentError,
};
use std::path::{Path, PathBuf};

use libparsec_types::prelude::*;

use crate::{get_local_pending_dir, LOCAL_PENDING_EXT};

#[derive(Debug, thiserror::Error)]
pub enum SavePkiLocalPendingError {
    #[error("No space available")]
    NoSpaceAvailable,
    #[error("invalid path")]
    InvalidPath,
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<SaveContentError> for SavePkiLocalPendingError {
    fn from(value: SaveContentError) -> Self {
        match value {
            SaveContentError::NotAFile
            | SaveContentError::InvalidParent
            | SaveContentError::InvalidPath
            | SaveContentError::ParentNotFound
            | SaveContentError::CannotEdit => SavePkiLocalPendingError::InvalidPath,

            SaveContentError::StorageNotAvailable | SaveContentError::NoSpaceLeft => {
                SavePkiLocalPendingError::NoSpaceAvailable
            }
            SaveContentError::Internal(_) => SavePkiLocalPendingError::Internal(value.into()),
        }
    }
}

pub async fn save_pki_local_pending(
    local_pending: PKILocalPendingEnrollment,
    local_file: PathBuf,
) -> Result<(), SavePkiLocalPendingError> {
    log::debug!(
        "Saving pki enrollment local pending file at {}",
        local_file.display()
    );
    let data = local_pending.dump();
    Ok(save_content(&local_file, &data).await?)
}

#[derive(Debug, thiserror::Error)]
enum LoadContentError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("storage not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<LoadFileError> for LoadContentError {
    fn from(value: LoadFileError) -> Self {
        match value {
            LoadFileError::StorageNotAvailable => LoadContentError::StorageNotAvailable,
            LoadFileError::NotAFile
            | LoadFileError::InvalidParent
            | LoadFileError::InvalidPath
            | LoadFileError::NotFound => LoadContentError::InvalidPath(value.into()),
            LoadFileError::Internal(error) => LoadContentError::Internal(error),
        }
    }
}

async fn load_pki_pending_file(path: &Path) -> Result<PKILocalPendingEnrollment, LoadContentError> {
    let content = load_file(path).await?;
    PKILocalPendingEnrollment::load(&content)
        .inspect_err(|err| log::debug!("Failed to load pki pending file {}: {err}", path.display()))
        .map_err(|_| LoadContentError::InvalidData)
}

#[derive(Debug, thiserror::Error)]
pub enum ListPkiLocalPendingError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(anyhow::Error),
}
impl From<ListFilesError> for ListPkiLocalPendingError {
    fn from(value: ListFilesError) -> Self {
        match value {
            ListFilesError::StorageNotAvailable => ListPkiLocalPendingError::StorageNotAvailable,
            ListFilesError::InvalidParent | ListFilesError::Internal(_) => {
                ListPkiLocalPendingError::Internal(value.into())
            }
        }
    }
}

pub async fn list_pki_local_pending(
    config_dir: &Path,
) -> Result<Vec<PKILocalPendingEnrollment>, ListPkiLocalPendingError> {
    let pending_dir = get_local_pending_dir(config_dir);
    let mut files = list_files(&pending_dir, LOCAL_PENDING_EXT).await?;

    // Sort entries so result is deterministic
    files.sort();
    log::trace!("Found pending request files: {files:?}");

    Ok(libparsec_platform_async::stream::iter(files)
        .filter_map(async |path| load_pki_pending_file(&path).await.ok())
        .collect::<Vec<_>>()
        .await)
}
