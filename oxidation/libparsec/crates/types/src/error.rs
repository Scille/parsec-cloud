// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use thiserror::Error;

use crate::{DateTime, DeviceID, EntryID, RealmID, UserID};
use libparsec_crypto::CryptoError;

#[derive(Error, Debug)]
pub enum EntryNameError {
    #[error("Name too long")]
    NameTooLong,
    #[error("Invalid name")]
    InvalidName,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DataError {
    #[error("Invalid compression")]
    Compression,

    #[error("{exc}")]
    Crypto { exc: CryptoError },

    #[error("Invalid serialization")]
    Serialization,

    #[error("Invalid signature")]
    Signature,

    #[error("Invalid HumanHandle")]
    InvalidHumanHandle,

    #[error("Invalid author: expected `{expected}`, got `{}`", match .got { Some(got) => got.to_string(), None => "None".to_string() })]
    UnexpectedAuthor {
        expected: DeviceID,
        got: Option<DeviceID>,
    },

    #[error("Invalid device ID: expected `{expected}`, got `{got}`")]
    UnexpectedDeviceID { expected: DeviceID, got: DeviceID },

    #[error("Invalid realm ID: expected `{expected}`, got `{got}`")]
    UnexpectedRealmID { expected: RealmID, got: RealmID },

    #[error("Invalid user ID: expected `{expected}`, got `{got}`")]
    UnexpectedUserID { expected: UserID, got: UserID },

    #[error("Invalid timestamp: expected `{expected}`, got `{got}`")]
    UnexpectedTimestamp { expected: DateTime, got: DateTime },

    #[error("Invalid entry ID: expected `{expected}`, got `{got}`")]
    UnexpectedId { expected: EntryID, got: EntryID },

    #[error("Invalid version: expected `{expected}`, got `{got}`")]
    UnexpectedVersion { expected: u32, got: u32 },
}

pub type DataResult<T> = Result<T, DataError>;

impl From<CryptoError> for DataError {
    fn from(exc: CryptoError) -> Self {
        Self::Crypto { exc }
    }
}
