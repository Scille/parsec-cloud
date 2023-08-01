// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use thiserror::Error;

use libparsec_types::prelude::*;

// TODO: remove me: seems only used in the python bindings
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
