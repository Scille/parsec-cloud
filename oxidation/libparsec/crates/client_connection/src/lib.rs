// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub mod authenticated_cmds;
pub mod client;
pub mod command_error;

pub use authenticated_cmds::AuthenticatedCmds;
pub use command_error::CommandError;

// TODO: use libparsec_testbed to handle testbed with no server !
