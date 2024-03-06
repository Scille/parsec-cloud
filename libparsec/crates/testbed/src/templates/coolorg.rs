// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

/// Coolorg contains:
/// - 3 users: `alice` (admin), `bob` (regular) and `mallory` (outsider)
/// - devices `alice@dev1`, `bob@dev1` and `mallory@dev1` whose storages are up to date
/// - devices `alice@dev2` and `bob@dev2` whose storages are empty
/// - 1 workspace `wksp1` shared between alice (owner) and bob (reader)
/// - `wksp1` is bootstrapped (i.e. has it initial key rotation and name certificates).
/// - Alice & Bob have their user realm created and user manifest v1 synced with `wksp1` in it
/// - Mallory has no user realm created
/// - 1 pending invitation from Alice for a new user with email `zack@example.invalid`
/// - 1 pending invitation for a new device for Alice
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    // If you change something here:
    // - Update this function's docstring
    // - Update `server/tests/common/client.py::CoolorgRpcClients`s docstring

    let mut builder = TestbedTemplate::from_builder("coolorg");

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

    // 2) Create user & device invitations

    builder.new_user_invitation("zack@example.invalid");
    builder.new_device_invitation("alice");

    // 3) Create workspace's realm shared between Alice&Bob, and it initial workspace manifest

    let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp1_id);
    let wksp1_name = builder
        .rename_realm(wksp1_id, "wksp1")
        .map(|e| e.name.clone());
    builder.share_realm(wksp1_id, "bob", Some(RealmRole::Reader));

    builder.store_stuff("wksp1_id", &wksp1_id);
    builder.store_stuff("wksp1_name", &wksp1_name);

    builder.create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id);

    // 4) Create users realms & and v1 manifest for Alice and Bob

    builder
        .new_user_realm("alice")
        .then_create_initial_user_manifest_vlob();

    builder
        .new_user_realm("bob")
        .then_create_initial_user_manifest_vlob();

    // 5) Initialize client storages for alice@dev1 and bob@dev1

    builder.certificates_storage_fetch_certificates("alice@dev1");
    builder.user_storage_fetch_user_vlob("alice@dev1");
    builder
        .user_storage_local_update("alice@dev1")
        .update_local_workspaces_with_fetched_certificates();
    builder.user_storage_fetch_realm_checkpoint("alice@dev1");
    builder.workspace_data_storage_fetch_workspace_vlob("alice@dev1", wksp1_id, None);
    builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", wksp1_id);

    builder.certificates_storage_fetch_certificates("bob@dev1");
    builder
        .user_storage_local_update("bob@dev1")
        .update_local_workspaces_with_fetched_certificates();
    builder.user_storage_fetch_realm_checkpoint("bob@dev1");
    builder.workspace_data_storage_fetch_workspace_vlob("bob@dev1", wksp1_id, None);
    builder.workspace_data_storage_fetch_realm_checkpoint("bob@dev1", wksp1_id);

    builder.certificates_storage_fetch_certificates("mallory@dev1");

    builder.finalize()
}
