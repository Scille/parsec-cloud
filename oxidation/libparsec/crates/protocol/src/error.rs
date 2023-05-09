// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    #[error("Encoding error: {exc}")]
    EncodingError { exc: String },
    #[error("Decoding error: {exc}")]
    DecodingError { exc: String },
    #[error("Invalid message")]
    InvalidMessageError,
    #[error("Message serialization error")]
    MessageSerializationError,
    #[error("Incompatible api version error")]
    IncompatibleAPIVersionsError,
    #[error("This protocol is not handled")]
    NotHandled,
    #[error("{exc}")]
    BadRequest { exc: String },
}
