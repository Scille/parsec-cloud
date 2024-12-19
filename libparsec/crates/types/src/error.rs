// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use thiserror::Error;

use libparsec_crypto::CryptoError;

use crate::{DateTime, DeviceFileType, DeviceID, HumanHandle, UserID, VlobID};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DataError {
    #[error("Invalid encryption")]
    Decryption,

    #[error("Invalid serialization: format {} step <{step}>", match .format { Some(format) => format!("{}", format), None => "<unknown>".to_string() })]
    BadSerialization {
        format: Option<u8>,
        step: &'static str,
    },

    #[error("Invalid signature")]
    Signature,

    #[error("Invalid author: expected `{expected}`, got `{}`", match .got { Some(got) => got.to_string(), None => "None".to_string() })]
    UnexpectedAuthor {
        expected: DeviceID,
        got: Option<DeviceID>,
    },

    #[error("Invalid author: expected root, got `{0}`")]
    UnexpectedNonRootAuthor(DeviceID),

    #[error("Invalid device ID: expected `{expected}`, got `{got}`")]
    UnexpectedDeviceID { expected: DeviceID, got: DeviceID },

    #[error("Invalid realm ID: expected `{expected}`, got `{got}`")]
    UnexpectedRealmID { expected: VlobID, got: VlobID },

    #[error("Invalid user ID: expected `{expected}`, got `{got}`")]
    UnexpectedUserID { expected: UserID, got: UserID },

    // `HumanHandle` is 72bytes long, so boxing is needed to limit presure on the stack
    #[error("Invalid HumanHandle, expected `{expected}`, got `{got}`")]
    UnexpectedHumanHandle {
        expected: Box<HumanHandle>,
        got: Box<HumanHandle>,
    },

    #[error("Invalid timestamp: expected `{expected}`, got `{got}`")]
    UnexpectedTimestamp { expected: DateTime, got: DateTime },

    #[error("Invalid entry ID: expected `{expected}`, got `{got}`")]
    UnexpectedId { expected: VlobID, got: VlobID },

    #[error("Invalid version: expected `{expected}`, got `{got}`")]
    UnexpectedVersion { expected: u32, got: u32 },

    #[error("Broken invariant in {data_type} content: {invariant}")]
    DataIntegrity {
        data_type: &'static str,
        invariant: &'static str,
    },

    #[error(transparent)]
    CryptoError(#[from] CryptoError),
}

pub type DataResult<T> = Result<T, DataError>;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PkiEnrollmentLocalPendingError {
    #[error("Cannot read {path}: {exc}")]
    CannotRead { path: PathBuf, exc: String },

    #[error("Cannot remove {path}: {exc}")]
    CannotRemove { path: PathBuf, exc: String },

    #[error("Cannot save {path}: {exc}")]
    CannotSave { path: PathBuf, exc: String },

    #[error("Cannot load local enrollment request: {exc}")]
    Validation { exc: DataError },
}

pub type PkiEnrollmentLocalPendingResult<T> = Result<T, PkiEnrollmentLocalPendingError>;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum LocalDeviceError {
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
    #[error("Device not found: {device_id}")]
    NotFound { device_id: DeviceID },

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
