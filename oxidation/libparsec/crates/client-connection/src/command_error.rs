use std::{error::Error, fmt::Display};

/// Sending a command isn't risk-free, we have multiple possible way to fail.
#[derive(Debug, PartialEq)]
pub enum CommandError {
    /// We failed to serialize the command.
    Serialization,
    /// We failed to retrieve the reply.
    Response,
    /// We failed to deserialize the reply.
    Deserialization,
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Serialization => write!(f, "failed to serialize the request"),
            CommandError::Response => write!(f, "error while retrieving the response"),
            CommandError::Deserialization => write!(f, "failed to deserialize the response"),
        }
    }
}

pub type Result<T> = core::result::Result<T, CommandError>;
