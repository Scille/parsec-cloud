// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::PathBuf;
use thiserror::Error;

use libparsec_crypto::CryptoError;

use crate::DeviceFileType;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LocalDeviceError {
    #[error("Could not access to the dir/file: {0}")]
    Access(PathBuf),

    #[error("Deserialization error: {0}")]
    Deserialization(PathBuf),

    #[error("Serialization error: {0}")]
    Serialization(PathBuf),

    #[error("Invalid slug")]
    InvalidSlug,

    #[error("Device key file `{0}` already exists")]
    AlreadyExists(PathBuf),

    #[error("{0}")]
    CryptoError(CryptoError),

    #[error("Not a Device {0:?} file")]
    Validation(DeviceFileType),
}

pub type LocalDeviceResult<T> = Result<T, LocalDeviceError>;
