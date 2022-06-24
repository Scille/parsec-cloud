// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::{error::Error, fmt::Display};

pub type Result<T> = core::result::Result<T, CommandError>;

/// Sending a command isn't risk-free, we have multiple possible way to fail.
/// Also note we only deal with *transport* related errors here (i.e. *deserialization* / *http* / *tcp* related stuff),
/// hence dealing with the `status` field of the response message is left to the caller
#[derive(Debug)]
pub enum CommandError {
    /// We failed to retrieve the reply.
    NoResponse(reqwest::Error),
    /// We receive a response but with an unexpected status code.
    InvalidResponseStatus(reqwest::StatusCode, reqwest::Response),
    /// We failed to deserialize the reply.
    Deserialization(libparsec_protocol::DecodeError),
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CommandError::InvalidResponseContent(e) => Some(e),
            CommandError::NoResponse(e) => Some(e),
            _ => None,
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::NoResponse(reason) => {
                write!(f, "failed to retrieving the response: {reason}")
            }
            CommandError::InvalidResponseStatus(code, _response) => {
                write!(f, "unexpected response status {code}")
            }
            CommandError::InvalidResponseContent(reason) => {
                write!(f, "failed to deserialize the response: {reason}")
            }
        }
    }
}

impl From<libparsec_protocol::DecodeError> for CommandError {
    fn from(e: libparsec_protocol::DecodeError) -> Self {
        Self::InvalidResponseContent(e)
    }
}

impl From<reqwest::Error> for CommandError {
    fn from(e: reqwest::Error) -> Self {
        Self::NoResponse(e)
    }
}
