// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use thiserror::Error;

use libparsec_crypto::CryptoError;

use crate::{DateTime, DeviceID, HumanHandle, UserID, VlobID};

pub use rmp_serde::{decode::Error as RmpDecodeError, encode::Error as RmpEncodeError};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DataError {
    #[error("Invalid encryption")]
    Decryption,

    #[error("Invalid serialization: format {} step <{step}>", match .format { Some(format) => format!("{format}"), None => "<unknown>".to_string() })]
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

    // `HumanHandle` is 72bytes long, so boxing is needed to limit pressure on the stack
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
