// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LocalDeviceError {
    #[error("Could not access to the dir/file: {0}")]
    Access(PathBuf),

    #[error("Deserialization error: {0}")]
    Deserialization(PathBuf),

    #[error("Serialization error: {0}")]
    Serialization(PathBuf),

    #[error("Invalid slug")]
    InvalidSlug,
}

pub type LocalDeviceResult<T> = Result<T, LocalDeviceError>;
