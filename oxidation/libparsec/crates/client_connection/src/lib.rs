// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod authenticated_cmds;
mod client;
mod error;

pub use authenticated_cmds::{AuthenticatedCmds, API_VERSION_HEADER_NAME, PARSEC_AUTH_METHOD};
pub use client::generate_client;
pub use error::{CommandError, CommandResult};
