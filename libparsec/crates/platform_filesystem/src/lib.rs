// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use anyhow::anyhow;
use std::io::ErrorKind;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;

#[derive(Debug, thiserror::Error)]
pub enum SaveContentError {
    #[error("storage not available")]
    StorageNotAvailable,
    #[error("not a file")]
    NotAFile,
    #[error("invalid parent")]
    InvalidParent,
    #[error("invalid path")]
    InvalidPath,
    #[error("parent not found")]
    ParentNotFound,
    #[error("cannot edit")]
    CannotEdit,
    #[error("no space left")]
    NoSpaceLeft,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl PartialEq for SaveContentError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SaveContentError::StorageNotAvailable, SaveContentError::StorageNotAvailable) => true,
            (SaveContentError::NotAFile, SaveContentError::NotAFile) => true,
            (SaveContentError::InvalidParent, SaveContentError::InvalidParent) => true,
            (SaveContentError::InvalidPath, SaveContentError::InvalidPath) => true,
            (SaveContentError::ParentNotFound, SaveContentError::ParentNotFound) => true,
            (SaveContentError::CannotEdit, SaveContentError::CannotEdit) => true,
            (SaveContentError::NoSpaceLeft, SaveContentError::NoSpaceLeft) => true,
            (SaveContentError::Internal(_), SaveContentError::Internal(_)) => true,

            (SaveContentError::StorageNotAvailable, _) => false,
            (SaveContentError::NotAFile, _) => false,
            (SaveContentError::InvalidParent, _) => false,
            (SaveContentError::InvalidPath, _) => false,
            (SaveContentError::ParentNotFound, _) => false,
            (SaveContentError::CannotEdit, _) => false,
            (SaveContentError::NoSpaceLeft, _) => false,
            (SaveContentError::Internal(_), _) => false,
        }
    }
}

impl From<std::io::Error> for SaveContentError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => SaveContentError::ParentNotFound,
            ErrorKind::PermissionDenied
            | ErrorKind::ReadOnlyFilesystem
            | ErrorKind::AlreadyExists => SaveContentError::CannotEdit,
            ErrorKind::NotADirectory => SaveContentError::InvalidParent,
            ErrorKind::IsADirectory => SaveContentError::NotAFile,
            ErrorKind::StorageFull
            | ErrorKind::QuotaExceeded
            | ErrorKind::FileTooLarge
            | ErrorKind::OutOfMemory => SaveContentError::NoSpaceLeft,

            e => SaveContentError::Internal(anyhow!("io error {e}")),
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
pub async fn save_content(path: &std::path::Path, content: &[u8]) -> Result<(), SaveContentError> {
    native::save_content(path, content).await
}

#[cfg(target_arch = "wasm32")]
pub async fn save_content(path: &std::path::Path, content: &[u8]) -> Result<(), SaveContentError> {
    let store = web::common::internal::Storage::new().await?;
    web::save_content(store, path, content).await
}

#[path = "../tests/units/mod.rs"]
#[cfg(test)]
mod tests;
