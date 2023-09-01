// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

/// Similar to minimal organization (i.e. single Alice user&device), but with some data:
/// - a workspace `wksp1` containing a folder `/foo` and a file `/bar.txt`
/// - folder `/foo` is empty
/// - file `/bar.txt` is composed of a single block with `hello world` as content
/// - Alice user, workspace and certificate storages are populated with all data
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    let mut builder = TestbedTemplate::from_builder("minimal_client_ready");

    // 1) Bootstrap organization & create Alice

    builder.bootstrap_organization("alice"); // alice@dev1

    // 2) Create workspace's realm

    let (wksp1_realm_id, wksp1_realm_key, realm_timestamp) = builder
        .new_realm("alice")
        .map(|e| (e.realm_id, e.realm_key.clone(), e.timestamp));

    // 3) Create `/foo`'s vlob

    let foo_id = builder
        .create_or_update_folder_manifest_vlob("alice@dev1", wksp1_realm_id, None)
        .map(|e| e.manifest.id);

    // 4) Create `/bar.txt`'s vlob & block

    let bar_txt_content = b"hello world";

    let bar_txt_block_access = builder
        .create_block("alice@dev1", wksp1_realm_id, bar_txt_content.as_ref())
        .as_block_access(0);
    let bar_txt_block_id = bar_txt_block_access.id;

    let bar_txt_id = builder
        .create_or_update_file_manifest_vlob("alice@dev1", wksp1_realm_id, None)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.size = bar_txt_block_access.size.get();
            manifest.blocks.push(bar_txt_block_access);
        })
        .map(|e| e.manifest.id);

    // 5) Add `/foo` & `/bar.txt` entries to the workspace manifest

    builder
        .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_realm_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.children.insert("foo".parse().unwrap(), foo_id);
            manifest
                .children
                .insert("bar.txt".parse().unwrap(), bar_txt_id);
        });

    // 6) Create Alice user realm & manifest

    builder
        .new_user_realm("alice")
        .then_create_initial_user_manifest_vlob()
        .customize(|e| {
            Arc::make_mut(&mut e.manifest)
                .workspaces
                .push(WorkspaceEntry::new(
                    wksp1_realm_id,
                    "wksp1".parse().unwrap(),
                    wksp1_realm_key,
                    1,
                    realm_timestamp,
                ))
        });

    // 7) Fetch all data in Alice's client storages

    builder.certificates_storage_fetch_certificates("alice@dev1");

    builder.user_storage_fetch_user_vlob("alice@dev1");
    builder.user_storage_fetch_realm_checkpoint("alice@dev1");

    let prevent_sync_pattern = Regex(vec![]);
    builder.workspace_data_storage_fetch_workspace_vlob(
        "alice@dev1",
        wksp1_realm_id,
        prevent_sync_pattern.clone(),
    );
    builder.workspace_data_storage_fetch_folder_vlob(
        "alice@dev1",
        wksp1_realm_id,
        foo_id,
        prevent_sync_pattern,
    );
    builder.workspace_data_storage_fetch_file_vlob("alice@dev1", wksp1_realm_id, bar_txt_id);
    builder.workspace_cache_storage_fetch_block("alice@dev1", wksp1_realm_id, bar_txt_block_id);
    builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", wksp1_realm_id);

    builder.finalize()
}
