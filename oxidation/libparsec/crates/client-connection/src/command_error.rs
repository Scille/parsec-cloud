use std::{error::Error, fmt::Display};

pub type Result<T> = core::result::Result<T, CommandError>;

/// Sending a command isn't risk-free, we have multiple possible way to fail.
#[derive(Debug)]
pub enum CommandError {
    /// We failed to serialize the command.
    Serialization(String),
    /// We failed to retrieve the reply.
    Response(reqwest::Error),
    /// We receive a response but with an unexpected status code.
    InvalidResponseStatus(reqwest::StatusCode, reqwest::Response),
    /// We failed to deserialize the reply.
    Deserialization(parsec_api_protocol::DecodeError),
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CommandError::Deserialization(e) => Some(e),
            CommandError::Response(e) => Some(e),
            CommandError::InvalidResponseStatus(_, _) | CommandError::Serialization(_) => None,
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Serialization(reason) => {
                write!(f, "failed to serialize the request: {reason}")
            }
            CommandError::Response(reason) => {
                write!(f, "failed to retrieving the response: {reason}")
            }
            CommandError::InvalidResponseStatus(code, _response) => {
                write!(f, "unexpected response status {code}")
            }
            CommandError::Deserialization(reason) => {
                write!(f, "failed to deserialize the response: {reason}")
            }
        }
    }
}

impl From<parsec_api_protocol::DecodeError> for CommandError {
    fn from(e: parsec_api_protocol::DecodeError) -> Self {
        Self::Deserialization(e)
    }
}

impl From<reqwest::Error> for CommandError {
    fn from(e: reqwest::Error) -> Self {
        Self::Response(e)
    }
}
