// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum MountpointError {
    #[error("Access denied")]
    AccessDenied,
    #[error("Dir is not empty")]
    DirNotEmpty,
    #[error("End of file")]
    EndOfFile,
    #[error("Name collision")]
    NameCollision,
    #[error("Not found")]
    NotFound,
}

pub(crate) type MountpointResult<T> = Result<T, MountpointError>;
