// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use crate::TestbedTemplate;

pub(crate) fn generate() -> Arc<TestbedTemplate> {
    // If you change something here:
    // - Update this function's docstring
    // - Update `server/tests/common/client.py::MinimalorgRpcClients`s docstring

    let mut builder = TestbedTemplate::from_builder("minimal");
    builder.add_account("alice@example.com");

    builder.finalize()
}
