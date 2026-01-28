// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use anyhow::anyhow;
use libparsec_types::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::{ffi::OsStr, path::PathBuf};

#[cfg(not(target_arch = "wasm32"))]
#[path = "native/mod.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
mod platform;

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

pub async fn save_content(path: &std::path::Path, content: &[u8]) -> Result<(), SaveContentError> {
    platform::save_content(path, content).await
}

#[derive(Debug, thiserror::Error)]
pub enum LoadFileError {
    #[error("storage not available")]
    StorageNotAvailable,
    #[error("not a file")]
    NotAFile,
    #[error("invalid parent")]
    InvalidParent,
    #[error("invalid path")]
    InvalidPath,
    #[error("file not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<std::io::Error> for LoadFileError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => LoadFileError::NotFound,
            ErrorKind::NotADirectory => LoadFileError::InvalidParent,
            ErrorKind::IsADirectory => LoadFileError::NotAFile,
            ErrorKind::InvalidFilename | ErrorKind::PermissionDenied => LoadFileError::InvalidPath,
            e => LoadFileError::Internal(anyhow!("io error {e}")),
        }
    }
}

pub async fn load_file(path: &Path) -> Result<Bytes, LoadFileError> {
    platform::load_file(path).await
}

#[derive(Debug, thiserror::Error)]
pub enum ListFilesError {
    #[error("storage not available")]
    StorageNotAvailable,
    #[error("invalid parent")]
    InvalidParent,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn list_files(
    root_dir: &Path,
    extension: impl AsRef<OsStr> + std::fmt::Display,
) -> Result<Vec<PathBuf>, ListFilesError> {
    platform::list_files(root_dir, extension).await
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveFileError {
    #[error("storage not available")]
    StorageNotAvailable,
    #[error("invalid parent")]
    InvalidParent,
    #[error("invalid path")]
    InvalidPath,
    #[error("not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<std::io::Error> for RemoveFileError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => RemoveFileError::NotFound,
            _ => RemoveFileError::Internal(value.into()),
        }
    }
}

pub async fn remove_file(path: &Path) -> Result<(), RemoveFileError> {
    platform::remove_file(path).await
}

#[derive(Debug, thiserror::Error)]
pub enum RenameFileError {
    #[error("storage not available")]
    StorageNotAvailable,
    #[error("no space left")]
    NoSpaceLeft,
    #[error("invalid parent")]
    InvalidParent,
    #[error("invalid path")]
    InvalidPath,
    #[error("not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<std::io::Error> for RenameFileError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => RenameFileError::NotFound,
            ErrorKind::StorageFull => RenameFileError::NoSpaceLeft,
            _ => RenameFileError::Internal(value.into()),
        }
    }
}

pub async fn rename_file(old: &Path, new: &Path) -> Result<(), RenameFileError> {
    platform::rename_file(old, new).await
}

#[path = "../tests/units/mod.rs"]
#[cfg(test)]
mod tests;
