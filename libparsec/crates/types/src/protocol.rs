// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

/*
 * ApiVersion
 */

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(from = "(u32, u32)", into = "(u32, u32)")]
pub struct ApiVersion {
    pub version: u32,
    pub revision: u32,
}

impl ApiVersion {
    pub fn dump(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(self)
    }

    pub fn load(buf: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(buf)
    }
}

impl PartialOrd for ApiVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ApiVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.version.cmp(&other.version) {
            Ordering::Equal => self.revision.cmp(&other.revision),
            order => order,
        }
    }
}

impl From<(u32, u32)> for ApiVersion {
    fn from(tuple: (u32, u32)) -> Self {
        Self {
            version: tuple.0,
            revision: tuple.1,
        }
    }
}

impl From<ApiVersion> for (u32, u32) {
    fn from(api_version: ApiVersion) -> Self {
        (api_version.version, api_version.revision)
    }
}

impl Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.version, self.revision)
    }
}

impl TryFrom<&str> for ApiVersion {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.split('.').count() != 2 {
            return Err(
                "Wrong number of `.` version string must be follow this pattern `<version>.<revision>`"
            );
        }

        let (version_str, revision_str) = value
            .split_once('.')
            .ok_or("Api version string must be follow this pattern `<version>.<revision>`")?;

        let version = version_str.parse::<u32>();
        let revision = revision_str.parse::<u32>();
        match (version, revision) {
            (Ok(a), Ok(b)) => Ok(ApiVersion {
                version: a,
                revision: b,
            }),
            _ => Err("Failed to parse version number (<version>.<revision>)"),
        }
    }
}

/*
 * ProtocolRequest
 */

/// There is two kinds of protocol family. On the one side, Anonymous, Invited and Authenticated
/// (with Tos being kind of like Authenticated) that are the regular families used to
/// interact between the metadata server and the client. And on the other side,
/// AuthenticatedAccount and AnonymousAccount that are used to store a device key file on
/// a server (typically used to use parsec's web version).
#[derive(Debug, Clone, Copy)]
pub enum ProtocolFamily {
    /// Family used for all requests done by a device
    Authenticated,
    /// Special case for requests done by a device before it has accepted the server's
    /// Terms Of Service (TOS)
    Tos,
    /// Family used for requests done without device (typically organization bootstrap)
    Anonymous,
    /// Family used by an invitation claimer in order to obtain a device
    Invited,
    /// Family used for non-authentication operations at server level.
    /// This is used to create a new Parsec account or get the server configuration.
    AnonymousServer,
    /// Family used for operations authenticated with Parsec account (ex: list organizations for a given account)
    AuthenticatedAccount,
}

pub trait ProtocolRequest<const V: u32> {
    const API_MAJOR_VERSION: u32 = V;
    const FAMILY: ProtocolFamily;
    type Response: for<'de> Deserialize<'de> + std::fmt::Debug;

    fn api_dump(&self) -> Result<Vec<u8>, ProtocolEncodeError>;

    fn api_load_response(buf: &[u8]) -> Result<Self::Response, ProtocolDecodeError>;
}

/// Error while deserializing data.
pub type ProtocolDecodeError = rmp_serde::decode::Error;
pub type ProtocolEncodeError = rmp_serde::encode::Error;
