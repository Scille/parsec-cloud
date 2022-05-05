// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod error;
mod handshake;

use parsec_serialization_format::parsec_cmds;

pub use error::*;
pub use handshake::*;

macro_rules! impl_dump_load {
    ($name:ident) => {
        impl $name {
            pub fn dump(&self) -> Result<Vec<u8>, &'static str> {
                ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
            }

            pub fn load(buf: &[u8]) -> Result<Self, &'static str> {
                ::rmp_serde::from_read_ref(buf).map_err(|_| "Deserialization failed")
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
// authenticated_cmds::AnyCmdReq::load(..)
// authenticated_cmds::block_create::Rep::Ok.dump()
// Client side
// authenticated_cmds::block_create::Req { .. }.dump()
// authenticated_cmds::block_create::Rep::load(..)
parsec_cmds!("schema/invited.json");
parsec_cmds!("schema/authenticated.json");
