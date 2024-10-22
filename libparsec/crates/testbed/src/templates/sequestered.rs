// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

/// Sequestered org contains:
/// - A sequestered organization obviously !
/// - 2 sequester services: `sequester_service_1` (revoked) and `sequester_service_2` (active)
/// - 2 users: `alice` (admin) and `bob` (regular)
/// - 2 devices: `alice@dev1` and `bob@dev1` (both device has its local storage
///   up-to-date regarding the certificates)
/// - 1 workspace `wksp1` shared between alice (owner) and bob (reader) containing a file `bar.txt`
///
/// The timeline regarding workspace and sequester services is as follows:
/// - Alice creates and bootstraps workspace `wksp1`
/// - Alice shares `wksp1` with Bob
/// - Alice uploads file manifest `bar.txt` in v1 (containing "Hello v1")
/// - Alice uploads workspace manifest in v1 (containing `bar.txt`)
/// - `sequester_service_1` is created
/// - Alice upload workspace manifest in v2 with renames `/bar.txt` -> `/bar2.txt`
/// - Alice does a key rotation on `wksp1` (key index becomes 2)
/// - Alice uploads file manifest `bar.txt` in v2 (containing "Hello v2")
/// - `sequester_service_1` is revoked
/// - Alice does a key rotation on `wksp1` (key index becomes 3)
/// - `sequester_service_2` is created
/// - Alice upload workspace manifest in v3 with renames `/bar.txt` -> `/bar3.txt`
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    // If you change something here:
    // - Update this function's docstring
    // - Update `server/tests/common/client.py::SequesteredOrgRpcClients`s docstring

    let mut builder = TestbedTemplate::from_builder("sequestered");

    // 1) Create user & devices

    builder
        .bootstrap_organization("alice") // alice@dev1
        .and_set_sequestered_organization();

    builder
        .new_user("bob")
        .with_initial_profile(UserProfile::Standard); // bob@dev1

    // Alice creates and bootstraps workspace `wksp1`

    let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp1_id);
    let wksp1_name = builder
        .rename_realm(wksp1_id, "wksp1")
        .map(|e| e.name.clone());

    builder.store_stuff("wksp1_id", &wksp1_id);
    builder.store_stuff("wksp1_name", &wksp1_name);

    // Alice shares `wksp1` with Bob

    builder.share_realm(wksp1_id, "bob", Some(RealmRole::Reader));

    // Alice uploads file manifest `bar.txt` in v1 (containing "Hello v1")

    let wksp1_bar_txt_id = builder.counters.next_entry_id();
    builder.store_stuff("wksp1_bar_txt_id", &wksp1_bar_txt_id);

    let bar_txt_v1_content = b"Hello v1";

    let bar_txt_v1_block_access = builder
        .create_block("alice@dev1", wksp1_id, bar_txt_v1_content.as_ref())
        .as_block_access(0);
    builder.store_stuff("wksp1_bar_txt_v1_block_access", &bar_txt_v1_block_access);

    builder
        .create_or_update_file_manifest_vlob(
            "alice@dev1",
            wksp1_id,
            Some(wksp1_bar_txt_id),
            wksp1_id,
        )
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.size = bar_txt_v1_block_access.size.get();
            manifest.blocks = vec![bar_txt_v1_block_access];
        });

    // Alice uploads workspace manifest in v1 (containing `bar.txt`)

    builder
        .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.children =
                HashMap::from_iter([("bar.txt".parse().unwrap(), wksp1_bar_txt_id)]);
        });

    // `sequester_service_1` is created

    let sequester_service_1_id = builder.new_sequester_service().map(|e| e.id);
    builder.store_stuff("sequester_service_1_id", &sequester_service_1_id);

    // Alice upload workspace manifest in v2 with renames `/bar.txt` -> `/bar2.txt`

    builder
        .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.children =
                HashMap::from_iter([("bar2.txt".parse().unwrap(), wksp1_bar_txt_id)]);
        });

    // Alice does a key rotation on `wksp1` (key index becomes 2)

    builder.rotate_key_realm(wksp1_id);

    // Alice uploads file manifest `bar.txt` in v2 (containing "Hello v2")

    let bar_txt_v2_content = b"Hello v2";

    let bar_txt_v2_block_access = builder
        .create_block("alice@dev1", wksp1_id, bar_txt_v2_content.as_ref())
        .as_block_access(0);
    builder.store_stuff("wksp1_bar_txt_v2_block_access", &bar_txt_v2_block_access);

    builder
        .create_or_update_file_manifest_vlob(
            "alice@dev1",
            wksp1_id,
            Some(wksp1_bar_txt_id),
            wksp1_id,
        )
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.size = bar_txt_v2_block_access.size.get();
            manifest.blocks = vec![bar_txt_v2_block_access];
        });

    // `sequester_service_1` is cancelled

    builder.revoke_sequester_service(sequester_service_1_id);

    // Alice does a key rotation on `wksp1` (key index becomes 3)

    builder.rotate_key_realm(wksp1_id);

    // `sequester_service_2` is created

    let sequester_service_2_id = builder.new_sequester_service().map(|e| e.id);
    builder.store_stuff("sequester_service_2_id", &sequester_service_2_id);

    // Alice upload workspace manifest in v3 with renames `/bar.txt` -> `/bar3.txt`

    builder
        .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.children =
                HashMap::from_iter([("bar3.txt".parse().unwrap(), wksp1_bar_txt_id)]);
        });

    // 3) Initialize client storages for alice@dev1 and bob@dev1

    macro_rules! fetch_and_update_local_storage {
        ($device: literal) => {
            builder.certificates_storage_fetch_certificates($device);
            builder
                .user_storage_local_update($device)
                .update_local_workspaces_with_fetched_certificates();
            builder.user_storage_fetch_realm_checkpoint($device);
            builder.workspace_data_storage_fetch_realm_checkpoint($device, wksp1_id);
        };
    }
    fetch_and_update_local_storage!("alice@dev1");
    fetch_and_update_local_storage!("bob@dev1");

    builder.finalize()
}
