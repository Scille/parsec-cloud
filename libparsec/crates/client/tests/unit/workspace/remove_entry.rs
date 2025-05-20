// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: remove opened file
// TODO: test remove non existing entry doesn't change updated or need_sync fields in parent manifest
// TODO: test remove on existing destination doesn't change updated or need_sync fields in parent manifest if overwrite is false

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls, ls, workspace_ops_factory};
use crate::workspace::{
    EntryStat, MoveEntryMode, OpenOptions, WorkspaceRemoveEntryError,
    store::ReadChunkOrBlockLocalOnlyError,
};

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    ops.remove_folder("/foo/spam".parse().unwrap())
        .await
        .unwrap();
    assert_ls!(ops, "/foo", ["egg.txt"]).await;

    ops.remove_file("/foo/egg.txt".parse().unwrap())
        .await
        .unwrap();
    assert_ls!(ops, "/foo", []).await;

    ops.remove_entry("/foo".parse().unwrap()).await.unwrap();
    ops.remove_entry("/bar.txt".parse().unwrap()).await.unwrap();

    assert_ls!(ops, "/", []).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_found(#[values("unknown", "parent_is_file")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let path = match kind {
        "unknown" => "/dummy",
        "parent_is_file" => "/bar.txt/dummy",
        unknown => panic!("Unknown kind: {}", unknown),
    };

    let err = ops.remove_folder(path.parse().unwrap()).await.unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryNotFound);

    let err = ops.remove_file(path.parse().unwrap()).await.unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryNotFound);

    let err = ops.remove_entry(path.parse().unwrap()).await.unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryNotFound);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn remove_file_but_is_folder(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .remove_file("/foo/spam".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryIsFolder);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn remove_folder_but_is_file(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .remove_folder("/foo/egg.txt".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryIsFile);

    let err = ops
        .remove_folder_all("/foo/egg.txt".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryIsFile);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn remove_not_empty_folder(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .remove_folder("/foo".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceRemoveEntryError::EntryIsNonEmptyFolder);

    ops.remove_folder_all("/foo".parse().unwrap())
        .await
        .unwrap();

    assert_ls!(ops, "/", ["bar.txt"]).await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ignore_invalid_children(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    env.customize(|builder| {
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                None,
            )
            .customize_children(
                [
                    // Patch `/foo` children entry so that they reference a manifest
                    // that disagree on who is their parent, hence making `/foo` an
                    // empty folder.
                    ("egg.txt", Some(wksp1_bar_txt_id)),
                    ("spam", Some(wksp1_foo_id)),
                ]
                .into_iter(),
            );
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    ops.remove_folder("/foo".parse().unwrap()).await.unwrap();

    assert_ls!(ops, "/", ["bar.txt"]).await;
}

// TODO: not supported yet, fix me !!!!
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn remove_file_with_local_changes(
    #[values(false, true)] removed_while_opened: bool,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Local changes

    let changes_chunk_view = {
        let fd = ops
            .open_file_by_id(wksp1_foo_egg_txt_id, OpenOptions::read_write())
            .await
            .unwrap();
        // `bar.txt` contains `hello world`, we overwrite part of it to `xxxxx world`
        ops.fd_write(fd, 0, b"xxxxx").await.unwrap();
        ops.fd_close(fd).await.unwrap();

        // The local file manifest now contains a dirty chunk containing `xxxxx`, then
        // a reference to the original chunk containing `hello world` (using only the
        // ` world` part).
        let manifest = ops.store.get_manifest(wksp1_foo_egg_txt_id).await.unwrap();
        match manifest {
            ArcLocalChildManifest::File(manifest) => {
                p_assert_eq!(manifest.blocks.len(), 1);
                p_assert_eq!(manifest.blocks[0].len(), 2);
                assert!(manifest.blocks[0][0].access.is_none());
                assert!(manifest.blocks[0][1].access.is_some());
                manifest.blocks[0][0].clone()
            }
            ArcLocalChildManifest::Folder(_) => unreachable!(),
        }
    };

    p_assert_eq!(
        ops.get_need_outbound_sync(u32::MAX).await.unwrap(),
        [wksp1_foo_egg_txt_id]
    );
    p_assert_eq!(
        ops.store
            .get_chunk_or_block_local_only(&changes_chunk_view)
            .await
            .unwrap(),
        b"xxxxx".as_ref()
    );

    // Now remove the file, which should also destroy the local changes

    if removed_while_opened {
        let fd = ops
            .open_file_by_id(wksp1_foo_egg_txt_id, OpenOptions::read_write())
            .await
            .unwrap();

        ops.remove_file("/foo/egg.txt".parse().unwrap())
            .await
            .unwrap();

        ops.fd_close(fd).await.unwrap();
    } else {
        ops.remove_file("/foo/egg.txt".parse().unwrap())
            .await
            .unwrap();
    }

    p_assert_eq!(
        ops.get_need_outbound_sync(u32::MAX).await.unwrap(),
        // Obviously the parent of the file has been modified !
        [wksp1_foo_id]
    );
    p_assert_matches!(
        ops.stat_entry_by_id(wksp1_foo_egg_txt_id).await.unwrap(),
        EntryStat::File { need_sync, .. } if !need_sync
    );
    p_assert_matches!(
        ops.store
            .get_chunk_or_block_local_only(&changes_chunk_view)
            .await
            .unwrap_err(),
        ReadChunkOrBlockLocalOnlyError::ChunkNotFound
    );
}

// TODO: not supported yet, fix me !!!!
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn remove_folder_with_local_changes(
    #[values("add_child", "rename_child")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Local changes

    match kind {
        "add_child" => {
            ops.create_file("/foo/new.txt".parse().unwrap())
                .await
                .unwrap();
        }
        "rename_child" => {
            ops.move_entry(
                "/foo/spam".parse().unwrap(),
                "/foo/spam2".parse().unwrap(),
                MoveEntryMode::NoReplace,
            )
            .await
            .unwrap();
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    p_assert_eq!(
        ops.get_need_outbound_sync(u32::MAX).await.unwrap(),
        [wksp1_foo_id]
    );

    // Now remove the folder, which should also destroy the local changes

    ops.remove_folder("/foo".parse().unwrap()).await.unwrap();

    p_assert_eq!(
        ops.get_need_outbound_sync(u32::MAX).await.unwrap(),
        // Obviously the parent of the folder has been modified !
        [wksp1_id]
    );
    p_assert_matches!(
        ops.stat_entry_by_id(wksp1_foo_id).await.unwrap(),
        EntryStat::Folder { need_sync, .. } if !need_sync
    );
}
