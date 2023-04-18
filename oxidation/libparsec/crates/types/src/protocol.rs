// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntegerBetween1And100(NonZeroU8);

impl TryFrom<u64> for IntegerBetween1And100 {
    type Error = &'static str;
    fn try_from(data: u64) -> Result<Self, Self::Error> {
        if data == 0 || data > 100 {
            return Err("Invalid IntegerBetween1And100 value");
        }

        Ok(Self(NonZeroU8::new(data as u8).unwrap()))
    }
}

impl From<IntegerBetween1And100> for u64 {
    fn from(data: IntegerBetween1And100) -> Self {
        u8::from(data.0) as u64
    }
}

pub trait ProtocolRequest {
    type Response: for<'de> Deserialize<'de>;

    fn dump(self) -> Result<Vec<u8>, ProtocolEncodeError>;

    fn load_response(buf: &[u8]) -> Result<Self::Response, ProtocolDecodeError>;
}

/// Error while deserializing data.
pub type ProtocolDecodeError = rmp_serde::decode::Error;
pub type ProtocolEncodeError = rmp_serde::encode::Error;
