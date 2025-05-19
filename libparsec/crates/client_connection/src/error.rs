// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use thiserror::Error;

use libparsec_types::prelude::*;

pub type ConnectionResult<T> = core::result::Result<T, ConnectionError>;

/// Connection errors regarding communication with the server
///
/// Sending a command isn't risk-free, there are multiple possible ways it can fail.
/// These are only *transport*-related errors (i.e. deserialization/http/tcp stuff)
/// including both client errors (4xx) and server errors (5xx).
/// Dealing with the response `status` field is left to the caller.
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

    /// Bad Accept header
    #[error("Bad Accept response received from the server (unexpected Accept header))")]
    BadAcceptType,

    /// Invitation already used or deleted
    #[error("Invitation already used or deleted")]
    InvitationAlreadyUsedOrDeleted,

    /// Bad content is sent by the server for the following reasons:
    /// - Bad content-type
    /// - Body is not a valid message
    /// - Unknown command
    #[error("Bad Content response received from the server (unexpected Content-Type header, body or RPC command)")]
    BadContent,

    /// The organization has expired
    #[error("The organization has expired")]
    ExpiredOrganization,

    /// Unexpected response status code sent by the server
    #[error("Invalid response status {0}")]
    InvalidResponseStatus(reqwest::StatusCode),

    /// Failed to deserialize the response
    #[error("Failed to deserialize the response: {0}")]
    InvalidResponseContent(ProtocolDecodeError),

    /// Server does not support API version used by the client but did not
    /// include the `Supported-Api-Versions` header in the response.
    #[error("Server does not support API version {api_version} but did not include the supported versions in the response")]
    MissingSupportedApiVersions { api_version: ApiVersion },

    /// Failed to retrieve the response
    #[error("Failed to retrieve the response: {}", .0.as_ref().map(reqwest::Error::to_string).unwrap_or_else(|| "Server unavailable".into()))]
    NoResponse(Option<reqwest::Error>),

    /// The user has been revoked
    #[error("User has been revoked")]
    RevokedUser,

    /// The user has been frozen (i.e. similar to revoked, but can be unfrozen)
    #[error("User has been frozen (temporarily suspended from the server)")]
    FrozenUser,

    /// The server requires the user to accept the Terms of Service (TOS)
    #[error("User must accept the Terms of Service to be able to connect to the server")]
    UserMustAcceptTos,

    /// The organization has been configured to refuse web clients (i.e.
    /// clients with `User-Agent` not starting with `Parsec-client/`).
    #[error("Web client is not allowed by the organization")]
    WebClientNotAllowedByOrganization,

    /// The authentication token has expired
    ///
    /// This corresponds to a 498 error returned by the server to indicate that
    /// the client has sent it signed authentication info with a date that's
    /// too old.
    ///
    /// Basically, it's triggered if the computer's clock is more than 5 minutes
    /// off the server's time (in theory, modern computers all use NTP to
    /// synchronize their clocks regularly, but a manual sync might be needed)
    #[error("Authentication token has expired. This could mean that the client's clock is too far behind or ahead of the server's")]
    AuthenticationTokenExpired,

    /// Server does not support the API version used by the client
    #[error("Server does not support API version {api_version}, supported versions are: {supported_api_versions:?}")]
    UnsupportedApiVersion {
        api_version: ApiVersion,
        supported_api_versions: Vec<ApiVersion>,
    },
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
        None => {
            log::error!("Missing Supported-Api-Versions header");
            ConnectionError::MissingSupportedApiVersions { api_version }
        }
    }
}
