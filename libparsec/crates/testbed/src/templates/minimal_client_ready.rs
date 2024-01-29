// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use crate::TestbedTemplate;

/// Similar to minimal organization (i.e. single Alice user&device), but with some data:
/// - A workspace `wksp1` that is bootstrapped (i.e. initial realm key rotation and rename have been done)
/// - The workspace contains a folder `/foo` and a file `/bar.txt`.
/// - File `/bar.txt` is composed of a single block with `hello world` as content.
/// - Folder `/foo` contains an empty folder `/foo/spam` and an empty file `/foo/egg.txt`.
/// - Device alice@dev1's user, workspace and certificate storages are populated with all data.
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    let mut builder = TestbedTemplate::from_builder("minimal_client_ready");

    // 1) Bootstrap organization & create Alice

    builder.bootstrap_organization("alice"); // alice@dev1

    // 2) Create workspace's realm

    let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp1_id);
    let wksp1_name = builder
        .rename_realm(wksp1_id, "wksp1")
        .map(|e| e.name.clone());

    builder.store_stuff("wksp1_id", &wksp1_id);
    builder.store_stuff("wksp1_name", &wksp1_name);

    // 3) Create `/bar.txt`'s vlob & block

    let bar_txt_content = b"hello world";

    let bar_txt_block_access = builder
        .create_block("alice@dev1", wksp1_id, bar_txt_content.as_ref())
        .as_block_access(0);
    let bar_txt_block_id = bar_txt_block_access.id;

    let bar_txt_id = builder
        .create_or_update_file_manifest_vlob("alice@dev1", wksp1_id, None)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.size = bar_txt_block_access.size.get();
            manifest.blocks.push(bar_txt_block_access);
        })
        .map(|e| e.manifest.id);

    builder.store_stuff("wksp1_bar_txt_id", &bar_txt_id);

    // 4) Create `/foo/spam`'s vlob

    let foo_spam_id = builder
        .create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, None)
        .map(|e| e.manifest.id);

    builder.store_stuff("wksp1_foo_spam_id", &foo_spam_id);

    // 5) Create `/foo/egg.txt`'s vlob

    let foo_egg_txt_id = builder
        .create_or_update_file_manifest_vlob("alice@dev1", wksp1_id, None)
        .map(|e| e.manifest.id);

    builder.store_stuff("wksp1_foo_egg_txt_id", &foo_egg_txt_id);

    // 6) Create `/foo`'s vlob and add it `/foo/spam` & `/foo/egg.txt` entries

    let foo_id = builder
        .create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, None)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest
                .children
                .insert("spam".parse().unwrap(), foo_spam_id);
            manifest
                .children
                .insert("egg.txt".parse().unwrap(), foo_egg_txt_id);
        })
        .map(|e| e.manifest.id);

    builder.store_stuff("wksp1_foo_id", &foo_id);

    // 7) Create workspace manifest's vlob and add it `/foo` & `/bar.txt` entries

    builder
        .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.children.insert("foo".parse().unwrap(), foo_id);
            manifest
                .children
                .insert("bar.txt".parse().unwrap(), bar_txt_id);
        });

    // 8) Create Alice user realm & manifest

    builder
        .new_user_realm("alice")
        .then_create_initial_user_manifest_vlob();

    // 9) Fetch all data in Alice's client storages

    builder.certificates_storage_fetch_certificates("alice@dev1");
    builder.user_storage_fetch_user_vlob("alice@dev1");
    builder
        .user_storage_local_update("alice@dev1")
        .update_local_workspaces_with_fetched_certificates();
    builder.user_storage_fetch_realm_checkpoint("alice@dev1");

    builder.workspace_data_storage_fetch_workspace_vlob("alice@dev1", wksp1_id, None);
    builder.workspace_data_storage_fetch_file_vlob("alice@dev1", wksp1_id, bar_txt_id);
    builder.workspace_cache_storage_fetch_block("alice@dev1", wksp1_id, bar_txt_block_id);
    builder.workspace_data_storage_fetch_folder_vlob("alice@dev1", wksp1_id, foo_id, None);
    builder.workspace_data_storage_fetch_folder_vlob("alice@dev1", wksp1_id, foo_spam_id, None);
    builder.workspace_data_storage_fetch_file_vlob("alice@dev1", wksp1_id, foo_egg_txt_id);
    builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", wksp1_id);

    builder.finalize()
}
