// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use thiserror::Error;

use libparsec_protocol::ApiVersion;

pub type CommandResult<T> = core::result::Result<T, CommandError>;

/// Sending a command isn't risk-free, we have multiple possible way to fail.
/// Also note we only deal with *transport* related errors here (i.e. *deserialization* / *http* / *tcp* related stuff),
/// hence dealing with the `status` field of the response message is left to the caller
#[derive(Debug, Error)]
pub enum CommandError {
    /// Any invalid content
    #[error("Invalid content")]
    BadContent,

    /// The organization has expired
    #[error("The organization has expired")]
    ExpiredOrganization,

    /// We receive a response but with an unexpected status code.
    #[error("Unexpected response status {0}")]
    InvalidResponseStatus(reqwest::StatusCode, reqwest::Response),

    /// We failed to deserialize the reply.
    #[error("Failed to deserialize the response: {0}")]
    InvalidResponseContent(libparsec_protocol::DecodeError),

    /// The invitation is already used/deleted
    #[error("Invalid handshake: Invitation already deleted")]
    InvitationAlreadyDeleted,

    /// We failed to retrieve the invitation
    #[error("Invalid handshake: Invitation not found")]
    InvitationNotFound,

    /// We failed to retrieve Api-Version
    #[error("Api-Version header is missing")]
    MissingApiVersion,

    /// We failed to retrieve Supported-Api-Versions
    #[error("Supported-Api-Versions header is missing")]
    MissingSupportedApiVersions,

    /// We failed to retrieve the reply.
    #[error("Failed to retrieving the response: {0}")]
    NoResponse(reqwest::Error),

    /// The user has beed revoked
    #[error("Device has been revoked")]
    RevokedUser,

    /// Failed to serialize the request
    #[error("{0}")]
    Serialization(libparsec_protocol::EncodeError),

    /// The version is not supported
    #[error("Unsupported API version: {api_version}, supported versions are: {supported_api_versions:?}")]
    UnsupportedApiVersion {
        api_version: ApiVersion,
        supported_api_versions: Vec<ApiVersion>,
    },

    /// We failed to deserialize ApiVersion
    #[error("Wrong ApiVersion {0}")]
    WrongApiVersion(String),
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

impl From<libparsec_protocol::EncodeError> for CommandError {
    fn from(e: libparsec_protocol::EncodeError) -> Self {
        Self::Serialization(e)
    }
}

pub(crate) fn unsupported_api_version_from_headers(
    headers: &reqwest::header::HeaderMap,
) -> CommandError {
    let api_version = match headers.get("Api-Version") {
        Some(api_version) => {
            let api_version = api_version.to_str().unwrap_or_default();
            match api_version.try_into() {
                Ok(api_version) => api_version,
                Err(_) => return CommandError::WrongApiVersion(api_version.into()),
            }
        }
        None => return CommandError::MissingApiVersion,
    };

    match headers.get("Supported-Api-Versions") {
        Some(supported_api_versions) => CommandError::UnsupportedApiVersion {
            api_version,
            supported_api_versions: supported_api_versions
                .to_str()
                .unwrap_or_default()
                .split(';')
                .filter_map(|x| ApiVersion::try_from(x).ok())
                .collect(),
        },
        None => CommandError::MissingSupportedApiVersions,
    }
}
