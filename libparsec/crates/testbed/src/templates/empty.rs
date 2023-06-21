// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use crate::TestbedTemplate;

/// Empty (i.e. not bootstrapped) organization
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    TestbedTemplate::from_builder("empty").finalize()
}
