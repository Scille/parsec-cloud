// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod error;
mod handshake;

use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

use serialization_format::parsec_cmds;

pub use error::*;
pub use handshake::*;

macro_rules! impl_dump_load {
    ($name:ident) => {
        impl $name {
            pub fn dump(&self) -> Result<Vec<u8>, &'static str> {
                ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
            }

            pub fn load(buf: &[u8]) -> Result<Self, &'static str> {
                ::rmp_serde::from_slice(buf).map_err(|_| "Deserialization failed")
            }
        }
    };
}
pub(crate) use impl_dump_load;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PerPage(NonZeroU8);

impl TryFrom<u64> for PerPage {
    type Error = &'static str;
    fn try_from(data: u64) -> Result<Self, Self::Error> {
        if data == 0 || data > 100 {
            return Err("Invalid PerPage value (between 1 and 100)");
        }

        Ok(Self(NonZeroU8::new(data as u8).unwrap()))
    }
}

impl From<PerPage> for u64 {
    fn from(data: PerPage) -> Self {
        u8::from(data.0) as u64
    }
}

// This macro implements dump/load methods for client/server side.
// It checks if both Req and Rep are implemented for a specified command
// It also provides a way to use commands by specifying status, command and type.
// For example:
// Server side
// authenticated_cmds::AnyCmdReq::load(..)
// authenticated_cmds::block_create::Rep::Ok.dump()
// Client side
// authenticated_cmds::block_create::Req { .. }.dump()
// authenticated_cmds::block_create::Rep::load(..)
parsec_cmds!("schema/invited_cmds");
parsec_cmds!("schema/authenticated_cmds");
