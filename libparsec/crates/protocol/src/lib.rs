// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod error;
mod version;

use libparsec_serialization_format::parsec_protocol_cmds_family;

pub use error::*;
pub use version::*;

// `libparsec_types` must be in the scope given the macro access it from submodule
// through `super::libparsec_types` (this is needed so that it can be mocked in tests)
#[allow(clippy::single_component_path_imports)]
use libparsec_types;

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
