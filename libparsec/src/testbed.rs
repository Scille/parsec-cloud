// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::{Path, PathBuf};

use libparsec_types::prelude::*;

#[cfg(feature = "test-utils")]
pub async fn test_new_testbed(template: &str, server_addr: Option<&BackendAddr>) -> PathBuf {
    libparsec_testbed::test_new_testbed(template, server_addr)
        .await
        .discriminant_dir
        .clone()
}

#[cfg(not(feature = "test-utils"))]
pub async fn test_new_testbed(_template: &str, _server_addr: Option<&BackendAddr>) -> PathBuf {
    panic!("Test features are disabled")
}

#[cfg(feature = "test-utils")]
pub async fn test_drop_testbed(discriminant_dir: &Path) {
    libparsec_testbed::test_drop_testbed(discriminant_dir).await
}

#[cfg(not(feature = "test-utils"))]
pub async fn test_drop_testbed(_path: &Path) {
    panic!("Test features are disabled")
}
