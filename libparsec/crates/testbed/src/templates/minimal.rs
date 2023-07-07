// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use crate::TestbedTemplate;

/// Minimal organization:
/// - Only a single alice user & device
/// - No data in local storage
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    let mut builder = TestbedTemplate::from_builder("minimal");

    builder.bootstrap_organization("alice");

    builder.finalize()
}
