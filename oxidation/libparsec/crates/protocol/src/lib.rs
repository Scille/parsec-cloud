// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod error;

use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};

use libparsec_serialization_format::parsec_protocol_cmds_family;

pub use error::*;

pub const API_V1_VERSION: ApiVersion = ApiVersion {
    version: 1,
    revision: 3,
};
pub const API_V2_VERSION: ApiVersion = ApiVersion {
    version: 2,
    revision: 5,
};
pub const API_VERSION: ApiVersion = API_V2_VERSION;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

// This macro implements dump/load methods for client/server side.
// It checks if both Req and Rep are implemented for a specified command
// It also provides a way to use commands by specifying status, command and type.
// For example:
// Server side
// authenticated_cmds::v2::AnyCmdReq::load(..)
// authenticated_cmds::v2::block_create::Rep::Ok.dump()
// Client side
// authenticated_cmds::v2::block_create::Req { .. }.dump()
// authenticated_cmds::v2::block_create::Rep::load(..)
parsec_protocol_cmds_family!("schema/invited_cmds");
parsec_protocol_cmds_family!("schema/authenticated_cmds");
parsec_protocol_cmds_family!("schema/anonymous_cmds");
