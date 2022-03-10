// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use crate::{DateTime, DeviceID};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Invalid encryption")]
    InvalidEncryption,
    #[error("Signature was forged or corrupt")]
    InvalidSignature,
    #[error("Invalid compression")]
    InvalidCompression,
    #[error("Invalid serializarion")]
    InvalidSerialization,
    #[error("Invalid author: expected `{expected}`, got `{got}`")]
    UnexpectedAuthor { expected: DeviceID, got: DeviceID },
    #[error("Invalid timestamp: expected `{expected}`, got `{got}`")]
    UnexpectedTimestamp { expected: DateTime, got: DateTime },
}
