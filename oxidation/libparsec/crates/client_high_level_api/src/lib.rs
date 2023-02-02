// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::PathBuf;

mod events;
mod fs;
mod invite;
mod login;
mod misc;

pub use events::*;
pub use fs::*;
pub use invite::*;
pub use login::*;
pub use misc::*;

#[cfg(feature = "test-utils")]
pub use libparsec_testbed::test_drop_testbed;

#[cfg(feature = "test-utils")]
pub async fn test_new_testbed(template: &str, server_addr: Option<&BackendAddr>) -> PathBuf {
    libparsec_testbed::test_new_testbed(template, server_addr)
        .await
        .client_config_dir
        .clone()
}

#[cfg(not(feature = "test-utils"))]
pub async fn test_new_testbed(_template: &str, _server_addr: Option<&BackendAddr>) -> PathBuf {
    panic!("Test features are disabled")
}

#[cfg(not(feature = "test-utils"))]
pub async fn test_drop_testbed(_path: &std::path::Path) {
    panic!("Test features are disabled")
}
