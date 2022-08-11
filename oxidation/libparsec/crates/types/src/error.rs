// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use thiserror::Error;

use crate::{DateTime, DeviceID};
use libparsec_crypto::CryptoError;

#[derive(Error, Debug)]
pub enum EntryNameError {
    #[error("Name too long")]
    NameTooLong,
    #[error("Invalid name")]
    InvalidName,
}

#[derive(Error, Debug, PartialEq)]
pub enum DataError {
    #[error("Invalid compression")]
    Compression,

    #[error("{exc}")]
    Crypto { exc: CryptoError },

    #[error("Invalid serialization")]
    Serialization,

    #[error("Invalid author: expected `{expected}`, got `{}`", match .got { Some(got) => got.to_string(), None => "None".to_string() })]
    UnexpectedAuthor {
        expected: DeviceID,
        got: Option<DeviceID>,
    },

    #[error("Invalid timestamp: expected `{expected}`, got `{got}`")]
    UnexpectedTimestamp { expected: DateTime, got: DateTime },
}

pub type DataResult<T> = Result<T, DataError>;

impl From<CryptoError> for DataError {
    fn from(exc: CryptoError) -> Self {
        Self::Crypto { exc }
    }
}
