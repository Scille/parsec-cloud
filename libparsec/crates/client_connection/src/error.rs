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
    #[error("Failed to retrieving the response: {}", .0.as_ref().map(reqwest::Error::to_string).unwrap_or_else(|| "Server unavailable".into()))]
    NoResponse(Option<reqwest::Error>),

    /// The user has beed revoked
    #[error("User has been revoked")]
    RevokedUser,

    /// The user has beed frozen (i.e. similar to revoked, but can be unfrozen)
    #[error("User has been frozen")]
    FrozenUser,

    /// The authentication token has expired
    #[error("Authentication token has expired")]
    AuthenticationTokenExpired,

    /// Failed to serialize the request
    #[error("{0}")]
    Serialization(ProtocolEncodeError),

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

// Custom equality to skip comparison of some fields
impl PartialEq for ConnectionError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BadContent, Self::BadContent) => true,
            (Self::ExpiredOrganization, Self::ExpiredOrganization) => true,
            // For the moment, InvalidResponseContentError are the same
            (Self::InvalidResponseContent(..), Self::InvalidResponseContent(..)) => true,
            // For the moment, InvalidResponseStatus are the same if they have the same status
            (
                Self::InvalidResponseStatus(left_status, ..),
                Self::InvalidResponseStatus(right_status, ..),
            ) => left_status == right_status,
            (Self::InvitationAlreadyDeleted, Self::InvitationAlreadyDeleted) => true,
            (Self::InvitationNotFound, Self::InvitationNotFound) => true,
            (Self::MissingApiVersion, Self::MissingApiVersion) => true,
            (Self::MissingSupportedApiVersions, Self::MissingSupportedApiVersions) => true,
            // For the moment, NoResponseError are the same
            (Self::NoResponse(..), Self::NoResponse(..)) => true,
            (Self::RevokedUser, Self::RevokedUser) => true,
            // For the moment, SerializationError are the same
            (Self::Serialization(..), Self::Serialization(..)) => true,
            (
                Self::UnsupportedApiVersion {
                    api_version: left_api_version,
                    ..
                },
                Self::UnsupportedApiVersion {
                    api_version: right_api_version,
                    ..
                },
            ) => left_api_version == right_api_version,
            (Self::WrongApiVersion(left), Self::WrongApiVersion(right)) => left == right,
            _ => false,
        }
    }
}

impl Eq for ConnectionError {}

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

impl From<ProtocolEncodeError> for ConnectionError {
    fn from(e: ProtocolEncodeError) -> Self {
        Self::Serialization(e)
    }
}

pub(crate) fn unsupported_api_version_from_headers(
    headers: &reqwest::header::HeaderMap,
) -> ConnectionError {
    let api_version = match headers.get("Api-Version") {
        Some(api_version) => {
            let api_version = api_version.to_str().unwrap_or_default();
            match api_version.try_into() {
                Ok(api_version) => api_version,
                Err(_) => return ConnectionError::WrongApiVersion(api_version.into()),
            }
        }
        None => return ConnectionError::MissingApiVersion,
    };

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
