// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use thiserror::Error;

use libparsec_crypto::CryptoError;

use crate::DeviceFileType;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LocalDeviceError {
    #[error("Invalid slug")]
    InvalidSlug,

    #[error("{exc}")]
    CryptoError { exc: CryptoError },

    #[error("Not a Device {ty:?} file")]
    Validation { ty: DeviceFileType },

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Device key file `{0}` already exists")]
    AlreadyExists(PathBuf),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Could not access to the dir/file: {0}")]
    Access(PathBuf),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Deserialization error: {0}")]
    Deserialization(PathBuf),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Serialization error: {0}")]
    Serialization(PathBuf),

    #[cfg(target_arch = "wasm32")]
    #[error("Device not found: {slug}")]
    NotFound { slug: String },

    #[cfg(target_arch = "wasm32")]
    #[error("LocalStorage is not available")]
    LocalStorageNotAvailable,
}

impl From<CryptoError> for LocalDeviceError {
    fn from(exc: CryptoError) -> Self {
        Self::CryptoError { exc }
    }
}

pub type LocalDeviceResult<T> = Result<T, LocalDeviceError>;
