// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use thiserror::Error;

use libparsec_types::EntryNameError;

#[derive(Debug, Error)]
pub enum MountpointError {
    #[error("Access denied")]
    AccessDenied,
    #[error("Dir is not empty")]
    DirNotEmpty,
    #[error("End of file")]
    EndOfFile,
    #[error("Invalid name")]
    InvalidName,
    #[error("Name collision")]
    NameCollision,
    #[error("Name too long")]
    NameTooLong,
    #[error("Not found")]
    NotFound,
}

pub type MountpointResult<T> = Result<T, MountpointError>;

impl From<EntryNameError> for MountpointError {
    fn from(value: EntryNameError) -> Self {
        match value {
            EntryNameError::NameTooLong => Self::NameTooLong,
            EntryNameError::InvalidEscapedValue => Self::InvalidName,
            EntryNameError::InvalidName => Self::InvalidName,
        }
    }
}
