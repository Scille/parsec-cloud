// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

/// Coolorg contains:
/// - 3 users: `alice` (admin), `bob` (regular) and `mallory` (outsider)
/// - devices `alice@dev1`, `bob@dev1` and `mallory@dev1` whose storages are up to date
/// - devices `alice@dev2` and `bob@dev2` whose storages are empty
/// - 1 workspace `wksp1` shared between alice (owner) and bob (reader)
/// - Alice & Bob have their user realm created and user manifest v1 synced with `wksp1` in it
/// - Mallory has not user realm created
/// - 1 pending invitation from Alice for a new user with email `zack@example.invalid`
/// - 1 pending invitation for a new device for Alice
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    let mut builder = TestbedTemplate::from_builder("coolorg");

    // 1) Create user & devices

    builder.bootstrap_organization("alice"); // alice@dev1
    builder.new_user("bob"); // bob@dev1
    builder.new_device("alice"); // alice@dev2
    builder.new_device("bob"); // bob@dev2
    builder.new_user("mallory"); // mallory@dev1

    // 2) Create user & device invitations

    builder.new_user_invitation("zack@example.invalid");
    builder.new_device_invitation("alice");

    // 3) Create workspace's realm shared between Alice&Bob, and it initial workspace manifest

    let (wksp1_id, wksp1_key, realm_timestamp) = {
        let event = builder.new_realm("alice");
        let wksp1_id = event.get_event().realm_id;
        let wksp1_key = event.get_event().realm_key.clone();
        let realm_timestamp = event.get_event().timestamp;
        event.then_share_with("bob", Some(RealmRole::Reader));
        (wksp1_id, wksp1_key, realm_timestamp)
    };

    builder.store_stuff("wksp1_id", &wksp1_id);
    builder.store_stuff("wksp1_key", &wksp1_key);

    builder.create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id);

    // 4) Create users realms & and v1 manifest for Alice and Bob

    let wksp1_entry = WorkspaceEntry::new(
        wksp1_id,
        "wksp1".parse().unwrap(),
        wksp1_key,
        1,
        realm_timestamp,
    );

    {
        let wksp1_entry = wksp1_entry.clone();
        builder
            .new_user_realm("alice")
            .then_create_initial_user_manifest_vlob()
            .customize(|e| Arc::make_mut(&mut e.manifest).workspaces.push(wksp1_entry));
    }

    builder
        .new_user_realm("bob")
        .then_create_initial_user_manifest_vlob()
        .customize(|e| Arc::make_mut(&mut e.manifest).workspaces.push(wksp1_entry));

    // 5) Initialize client storages for alice@dev1 and bob@dev1

    builder.certificates_storage_fetch_certificates("alice@dev1");
    builder.user_storage_fetch_user_vlob("alice@dev1");
    builder.user_storage_fetch_realm_checkpoint("alice@dev1");
    builder.workspace_data_storage_fetch_workspace_vlob("alice@dev1", wksp1_id, None);
    builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", wksp1_id);

    builder.certificates_storage_fetch_certificates("bob@dev1");
    builder.user_storage_fetch_user_vlob("bob@dev1");
    builder.user_storage_fetch_realm_checkpoint("bob@dev1");
    builder.workspace_data_storage_fetch_workspace_vlob("bob@dev1", wksp1_id, None);
    builder.workspace_data_storage_fetch_realm_checkpoint("bob@dev1", wksp1_id);

    builder.certificates_storage_fetch_certificates("mallory@dev1");

    builder.finalize()
}
