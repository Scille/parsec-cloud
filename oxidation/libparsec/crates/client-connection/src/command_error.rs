use std::{error::Error, fmt::Display};

/// Sending a command isn't risk-free, we have multiple possible way to fail.
#[derive(Debug)]
pub enum CommandError {
    /// We failed to serialize the command.
    Serialization(String),
    /// We failed to retrieve the reply.
    Response(reqwest::Error),
    /// We failed to deserialize the reply.
    Deserialization(parsec_api_protocol::DecodeError),
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Serialization(reason) => {
                write!(f, "failed to serialize the request: {reason}")
            }
            CommandError::Response(reason) => {
                write!(f, "failed to retrieving the response: {reason}")
            }
            CommandError::Deserialization(reason) => {
                write!(f, "failed to deserialize the response: {reason}")
            }
        }
    }
}

pub type Result<T> = core::result::Result<T, CommandError>;
