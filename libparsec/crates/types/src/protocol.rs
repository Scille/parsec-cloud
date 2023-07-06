// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{cmp::Ordering, fmt::Display, num::NonZeroU8};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntegerBetween1And100(NonZeroU8);

impl TryFrom<u64> for IntegerBetween1And100 {
    type Error = &'static str;
    fn try_from(data: u64) -> Result<Self, Self::Error> {
        if data == 0 || data > 100 {
            return Err("Invalid IntegerBetween1And100 value");
        }

        Ok(Self(
            NonZeroU8::new(data as u8).expect("The value is in the boundary of an u8"),
        ))
    }
}

impl From<IntegerBetween1And100> for u64 {
    fn from(data: IntegerBetween1And100) -> Self {
        u8::from(data.0) as u64
    }
}

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

/*
 * ProtocolRequest
 */

pub trait ProtocolRequest<const V: u32> {
    const API_MAJOR_VERSION: u32 = V;
    type Response: for<'de> Deserialize<'de>;

    fn api_dump(&self) -> Result<Vec<u8>, ProtocolEncodeError>;

    fn api_load_response(buf: &[u8]) -> Result<Self::Response, ProtocolDecodeError>;
}

/// Error while deserializing data.
pub type ProtocolDecodeError = rmp_serde::decode::Error;
pub type ProtocolEncodeError = rmp_serde::encode::Error;
