// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use thiserror::Error;

use libparsec_types::prelude::*;

pub type ConnectionResult<T> = core::result::Result<T, ConnectionError>;

/// Sending a command isn't risk-free, we have multiple possible way to fail.
/// Also note we only deal with *transport* related errors here (i.e. *deserialization* / *http* / *tcp* related stuff),
/// hence dealing with the `status` field of the response message is left to the caller
#[derive(Debug, Error)]
pub enum ConnectionError {
    /// Missing authentication info
    #[error("Missing authentication info")]
    MissingAuthenticationInfo,

    /// Bad authentication info
    #[error("Bad authentication info")]
    BadAuthenticationInfo,

    /// Organization not found
    #[error("Organization not found")]
    OrganizationNotFound,

    /// Bad accept type
    #[error("Bad accept type")]
    BadAcceptType,

    /// Invitation already used or deleted
    #[error("Invitation already used or deleted")]
    InvitationAlreadyUsedOrDeleted,

    /// Any invalid content
    #[error("Invalid content")]
    BadContent,

    /// The organization has expired
    #[error("The organization has expired")]
    ExpiredOrganization,

    /// We receive a response but with an unexpected status code.
    #[error("Unexpected response status {0}")]
    InvalidResponseStatus(reqwest::StatusCode),

    /// We failed to deserialize the reply.
    #[error("Failed to deserialize the response: {0}")]
    InvalidResponseContent(ProtocolDecodeError),

    /// We failed to retrieve Supported-Api-Versions
    #[error("Supported-Api-Versions header is missing")]
    MissingSupportedApiVersions,

    /// We failed to retrieve the reply.
    #[error("Failed to retrieving the response: {}", .0.as_ref().map(reqwest::Error::to_string).unwrap_or_else(|| "Server unavailable".into()))]
    NoResponse(Option<reqwest::Error>),

    /// The user has beed revoked
    #[error("User has been revoked")]
    RevokedUser,

    /// The user has beed frozen (i.e. similar to revoked, but can be unfrozen)
    #[error("User has been frozen")]
    FrozenUser,

    /// The server requires the user to accept the Terms of Service (TOS)
    #[error("User must first accept the Terms of Service")]
    UserMustAcceptTos,

    /// The authentication token has expired
    #[error("Authentication token has expired")]
    AuthenticationTokenExpired,

    /// The version is not supported
    #[error("Unsupported API version: {api_version}, supported versions are: {supported_api_versions:?}")]
    UnsupportedApiVersion {
        api_version: ApiVersion,
        supported_api_versions: Vec<ApiVersion>,
    },

    /// We failed to deserialize ApiVersion
    #[error("Wrong ApiVersion {0}")]
    WrongApiVersion(String),

    #[error("Invalid sse event id: {0}")]
    InvalidSSEEventID(#[from] reqwest::header::InvalidHeaderValue),
}

impl From<ProtocolDecodeError> for ConnectionError {
    fn from(e: ProtocolDecodeError) -> Self {
        Self::InvalidResponseContent(e)
    }
}

impl From<reqwest::Error> for ConnectionError {
    fn from(e: reqwest::Error) -> Self {
        Self::NoResponse(Some(e))
    }
}

pub(crate) fn unsupported_api_version_from_headers(
    api_version: ApiVersion,
    headers: &reqwest::header::HeaderMap,
) -> ConnectionError {
    match headers.get("Supported-Api-Versions") {
        Some(supported_api_versions) => ConnectionError::UnsupportedApiVersion {
            api_version,
            supported_api_versions: supported_api_versions
                .to_str()
                .unwrap_or_default()
                .split(';')
                .filter_map(|x| ApiVersion::try_from(x).ok())
                .collect(),
        },
        None => ConnectionError::MissingSupportedApiVersions,
    }
}
