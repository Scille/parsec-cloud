// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod error;
mod handshake;

use serialization_format::parsec_protocol;

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
parsec_protocol!("schema/invited_cmds");
parsec_protocol!("schema/authenticated_cmds");
