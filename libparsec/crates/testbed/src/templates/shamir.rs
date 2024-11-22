// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

/// Shamir org contains:
/// - 4 users: `alice` (admin), `bob` (standard), `mallory` (outsider) and `mike` (standard)
/// - Alice has two devices: `alice@dev1` and `alice@dev2`
/// - Bob has two devices: `bob@dev1` and `bob@dev2`
/// - Mallory has two devices: `mallory@dev1` and `mallory@dev2`
/// - Mike only has one device: `mike@dev1`
/// - Alice has a shamir recovery setup (threshold: 2, recipients: Bob with 2 shares,
///   Mallory & mike with 1 share each)
/// - Bob has a deleted shamir recovery (used to be threshold: 1, recipients: Alice & Mallory)
/// - Mallory has a shamir recovery setup (threshold: 1, recipients: only Mike)
/// - Bob has invited Alice to do a Shamir recovery
/// - devices `alice@dev1`/`bob@dev1`/`mallory@dev1`/`mike@dev1` starts with up-to-date storages
/// - devices `alice@dev2` and `bob@dev2` whose storages are empty
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    // If you change something here:
    // - Update this function's docstring
    // - Update `server/tests/common/client.py::ShamirOrgRpcClients`s docstring

    let mut builder = TestbedTemplate::from_builder("shamir");

    // 1) Create user & devices

    builder.bootstrap_organization("alice"); // alice@dev1
    builder
        .new_user("bob")
        .with_initial_profile(UserProfile::Standard); // bob@dev1
    builder.new_device("alice"); // alice@dev2
    builder.new_device("bob"); // bob@dev2
    builder
        .new_user("mallory")
        .with_initial_profile(UserProfile::Outsider); // mallory@dev1
    builder.new_device("mallory"); // mallory@dev2
    builder
        .new_user("mike")
        .with_initial_profile(UserProfile::Standard); // mike@dev1

    // 2) Create shamir recovery

    builder.new_shamir_recovery(
        "alice",
        2,
        [
            ("bob".parse().unwrap(), 2.try_into().unwrap()),
            ("mallory".parse().unwrap(), 1.try_into().unwrap()),
            ("mike".parse().unwrap(), 1.try_into().unwrap()),
        ],
        "alice@dev2",
    );

    builder.new_shamir_recovery(
        "bob",
        1,
        [
            ("alice".parse().unwrap(), 1.try_into().unwrap()),
            ("mallory".parse().unwrap(), 1.try_into().unwrap()),
        ],
        "bob@dev2",
    );
    builder.delete_shamir_recovery("bob");

    builder.new_shamir_recovery(
        "mallory",
        1,
        [("mike".parse().unwrap(), 1.try_into().unwrap())],
        "mallory@dev2",
    );

    builder.new_shamir_recovery_invitation("alice");

    // 3) Initialize client storages for alice@dev1/bob@dev1/mallory@dev1/mike@dev1

    builder.certificates_storage_fetch_certificates("alice@dev1");
    builder.certificates_storage_fetch_certificates("bob@dev1");
    builder.certificates_storage_fetch_certificates("mallory@dev1");
    builder.certificates_storage_fetch_certificates("mike@dev1");

    builder.finalize()
}
