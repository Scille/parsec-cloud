// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use crate::TestbedTemplate;

pub(crate) fn generate() -> Arc<TestbedTemplate> {
    let mut builder = TestbedTemplate::from_builder("coolorg");

    builder.bootstrap_organization("alice");
    builder.new_user("bob"); // bob@dev1
    builder.new_device("alice"); // alice@dev2
    builder.new_device("bob"); // bob@dev2
    builder
        .new_user("mallory") // mallory@dev1
        .with_human_handle(None)
        .with_first_device_label(None);

    builder.finalize()
}
