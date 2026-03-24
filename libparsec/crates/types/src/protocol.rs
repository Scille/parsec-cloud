// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

pub use libparsec_serialization_format_types::{ProtocolFamily, ProtocolRequest};

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

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseApiVersionError {
    #[error("Api version string must be follow this pattern `<version>.<revision>`")]
    MissingSeparator,
    #[error("Invalid version number: {0}")]
    InvalidVersionNumber(std::num::ParseIntError),
    #[error("Invalid revision number: {0}")]
    InvalidRevisionNumber(std::num::ParseIntError),
}

impl TryFrom<&str> for ApiVersion {
    type Error = ParseApiVersionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (version_str, revision_str) = value
            .split_once('.')
            .ok_or(ParseApiVersionError::MissingSeparator)?;

        let version = version_str
            .parse::<u32>()
            .map_err(ParseApiVersionError::InvalidVersionNumber)?;
        let revision = revision_str
            .parse::<u32>()
            .map_err(ParseApiVersionError::InvalidRevisionNumber)?;
        Ok(ApiVersion { version, revision })
    }
}

/// Error while deserializing data.
pub type ProtocolDecodeError = rmp_serde::decode::Error;
pub type ProtocolEncodeError = rmp_serde::encode::Error;
