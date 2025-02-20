// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_realm_get_keys_bundle, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::{
    workspace::{InboundSyncOutcome, OpenOptions},
    EventWorkspaceOpsInboundSyncDone,
};

enum RemoteModification {
    Nothing,
    Overwritten,
    ReParented,
}

enum LocalModification {
    Nothing,
    Conflicting,
    ConflictingAndNameClash,
}

// TODO: test with parent as folder manifest or as workspace manifest

#[parsec_test(testbed = "minimal_client_ready")]
async fn non_placeholder(
    #[values(
        RemoteModification::Nothing,
        RemoteModification::Overwritten,
        RemoteModification::ReParented,
        // TODO: New vlob contains corrupted data
        // TODO: New vlob contains a folder manifest instead of the expected file
    )]
    remote_modification: RemoteModification,
    #[values(
        LocalModification::Nothing,
        LocalModification::Conflicting,
        LocalModification::ConflictingAndNameClash
    )]
    local_modification: LocalModification,
    env: &TestbedEnv,
) {
    if matches!(
        (&local_modification, &remote_modification),
        (
            LocalModification::Conflicting | LocalModification::ConflictingAndNameClash,
            RemoteModification::Nothing
        ) | (
            LocalModification::ConflictingAndNameClash,
            RemoteModification::ReParented
        )
    ) {
        // Meaningless case, just skip it
        return;
    }

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");

    // 1) Customize testbed

    env.customize(|builder| {
        builder.new_device("alice"); // alice@dev2
        builder.certificates_storage_fetch_certificates("alice@dev1");

        match remote_modification {
            RemoteModification::Nothing => (),
            RemoteModification::Overwritten => {
                let block_data = b"abc";
                let wksp1_bar_txt_new_block_id = builder
                    .create_block("alice@dev2", wksp1_id, block_data.as_ref())
                    .map(|e| e.block_id);
                builder.store_stuff("wksp1_bar_txt_new_block_id", &wksp1_bar_txt_new_block_id);
                builder
                    .create_or_update_file_manifest_vlob(
                        "alice@dev2",
                        wksp1_id,
                        wksp1_bar_txt_id,
                        None,
                    )
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.manifest);
                        manifest.blocks.clear();
                        manifest.size = block_data.len() as u64;
                        manifest.blocks.push(BlockAccess {
                            id: wksp1_bar_txt_new_block_id,
                            offset: 0,
                            size: manifest.size.try_into().unwrap(),
                            digest: HashDigest::from_data(block_data),
                        });
                    });
            }
            RemoteModification::ReParented => {
                builder
                    .create_or_update_folder_manifest_vlob(
                        "alice@dev2",
                        wksp1_id,
                        wksp1_foo_id,
                        None,
                    )
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.manifest);
                        manifest
                            .children
                            .insert("bar.txt".parse().unwrap(), wksp1_bar_txt_id);
                    });
                builder
                    .create_or_update_file_manifest_vlob(
                        "alice@dev2",
                        wksp1_id,
                        wksp1_bar_txt_id,
                        None,
                    )
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.manifest);
                        manifest.parent = wksp1_foo_id;
                    });
            }
        };

        match (&local_modification, &remote_modification) {
            (LocalModification::Nothing, _) => (),

            (
                LocalModification::Conflicting | LocalModification::ConflictingAndNameClash,
                RemoteModification::Overwritten,
            ) => {
                let wksp1_bar_txt_new_chunk_id = builder
                    .workspace_data_storage_chunk_create("alice@dev1", wksp1_id, b"ABC".as_ref())
                    .map(|e| e.chunk_id);
                builder.store_stuff("wksp1_bar_txt_new_chunk_id", &wksp1_bar_txt_new_chunk_id);
                builder
                    .workspace_data_storage_local_file_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_bar_txt_id,
                        None,
                    )
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.local_manifest);
                        manifest.need_sync = true;
                        manifest.blocks.clear();
                        manifest.size = 3;
                        manifest.blocks.push(vec![ChunkView {
                            id: wksp1_bar_txt_new_chunk_id,
                            start: 0,
                            stop: 3.try_into().unwrap(),
                            raw_offset: 0,
                            raw_size: 3.try_into().unwrap(),
                            access: None,
                        }]);
                    });

                if matches!(
                    local_modification,
                    LocalModification::ConflictingAndNameClash
                ) {
                    let wksp1_bar2_txt_id = builder
                        .workspace_data_storage_local_file_manifest_create_or_update(
                            "alice@dev1",
                            wksp1_id,
                            None,
                            wksp1_id,
                        )
                        .map(|e| e.local_manifest.base.id);
                    builder.store_stuff("wksp1_bar2_txt_id", &wksp1_bar2_txt_id);
                    builder
                        .workspace_data_storage_local_workspace_manifest_update(
                            "alice@dev1",
                            wksp1_id,
                        )
                        .customize(|e| {
                            let manifest = Arc::make_mut(&mut e.local_manifest);
                            manifest.children.insert(
                                "bar (Parsec sync conflict 2).txt".parse().unwrap(),
                                wksp1_bar2_txt_id,
                            );
                        });
                }
            }

            (LocalModification::Conflicting, RemoteModification::ReParented) => {
                builder
                    .workspace_data_storage_local_folder_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_foo_spam_id,
                        None,
                    )
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.local_manifest);
                        manifest
                            .children
                            .insert("bar.txt".parse().unwrap(), wksp1_bar_txt_id);
                    });
                builder
                    .workspace_data_storage_local_file_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_bar_txt_id,
                        None,
                    )
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.local_manifest);
                        manifest.parent = wksp1_foo_spam_id;
                    });
            }

            (
                LocalModification::Conflicting | LocalModification::ConflictingAndNameClash,
                RemoteModification::Nothing,
            )
            | (LocalModification::ConflictingAndNameClash, RemoteModification::ReParented) => {
                unreachable!()
            }
        }
    })
    .await;

    // Get back last workspace manifest version synced in server
    let (wksp1_bar_txt_last_remote_manifest, wksp1_bar_txt_last_encrypted) = env
        .template
        .events
        .iter()
        .rev()
        .find_map(|e| match e {
            TestbedEvent::CreateOrUpdateFileManifestVlob(e)
                if e.manifest.id == wksp1_bar_txt_id =>
            {
                Some((e.manifest.clone(), e.encrypted(&env.template)))
            }
            _ => None,
        })
        .unwrap();

    // 2) Start workspace ops

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let before_sync_bar_txt_manifest = match wksp1_ops
        .store
        .get_manifest(wksp1_bar_txt_id)
        .await
        .unwrap()
    {
        ArcLocalChildManifest::File(manifest) => manifest,
        ArcLocalChildManifest::Folder(_) => unreachable!(),
    };

    // 3) Actual sync operation

    // Mock server command `vlob_read` fetch the last version (i.e. v1 for
    // `RemoteModification::Nothing`, v2 else)of the workspace manifest

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Read the file manifest's vlob
        {
            let wksp1_bar_txt_last_remote_manifest = wksp1_bar_txt_last_remote_manifest.clone();
            let last_realm_certificate_timestamp =
                env.get_last_realm_certificate_timestamp(wksp1_id);
            let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
            move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
                p_assert_eq!(req.at, None);
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.vlobs, [wksp1_bar_txt_id]);
                authenticated_cmds::latest::vlob_read_batch::Rep::Ok {
                    items: vec![(
                        wksp1_bar_txt_id,
                        1,
                        wksp1_bar_txt_last_remote_manifest.author,
                        wksp1_bar_txt_last_remote_manifest.version,
                        wksp1_bar_txt_last_remote_manifest.timestamp,
                        wksp1_bar_txt_last_encrypted,
                    )],
                    needed_common_certificate_timestamp: last_common_certificate_timestamp,
                    needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                }
            }
        },
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    let mut spy = wksp1_ops.event_bus.spy.start_expecting();
    wksp1_ops.inbound_sync(wksp1_bar_txt_id).await.unwrap();
    if matches!(&remote_modification, RemoteModification::Nothing) {
        spy.assert_no_events();
    } else {
        let expected_parent_id = if matches!(&remote_modification, RemoteModification::ReParented) {
            wksp1_foo_id
        } else {
            wksp1_id
        };
        spy.assert_next(|e| {
            p_assert_matches!(e, EventWorkspaceOpsInboundSyncDone { realm_id, entry_id, parent_id }
                if *realm_id == wksp1_id
                && *entry_id == wksp1_bar_txt_id
                && *parent_id == expected_parent_id
            )
        });
    }

    let bar_txt_manifest = match wksp1_ops
        .store
        .get_manifest(wksp1_bar_txt_id)
        .await
        .unwrap()
    {
        ArcLocalChildManifest::File(m) => m,
        m => panic!(
            "Invalid manifest type for `/bar.txt`, expecting file and got: {:?}",
            m
        ),
    };

    // 4) Check the outcome

    p_assert_eq!(bar_txt_manifest.base, *wksp1_bar_txt_last_remote_manifest);
    p_assert_eq!(bar_txt_manifest.need_sync, false);

    let parent_manifest = wksp1_ops.store.get_root_manifest();
    p_assert_eq!(
        *parent_manifest
            .children
            .get(&"bar.txt".parse().unwrap())
            .unwrap(),
        wksp1_bar_txt_id,
    );

    // Reparenting doesn't lead to a conflict
    if matches!(&local_modification, LocalModification::Conflicting)
        && !matches!(&remote_modification, RemoteModification::ReParented)
    {
        let conflicted_id = *parent_manifest
            .children
            .get(&"bar (Parsec sync conflict 2).txt".parse().unwrap())
            .unwrap();
        p_assert_ne!(conflicted_id, wksp1_bar_txt_id);
        let conflicted_manifest = match wksp1_ops.store.get_manifest(conflicted_id).await.unwrap() {
            ArcLocalChildManifest::File(manifest) => manifest,
            ArcLocalChildManifest::Folder(manifest) => panic!("Expected file, got {:?}", manifest),
        };

        let LocalFileManifest {
            base,
            parent,
            need_sync,
            updated,
            size,
            blocksize,
            blocks,
        } = conflicted_manifest.as_ref();
        p_assert_eq!(*parent, before_sync_bar_txt_manifest.parent);
        p_assert_eq!(*need_sync, before_sync_bar_txt_manifest.need_sync);
        p_assert_eq!(*updated, before_sync_bar_txt_manifest.updated);
        p_assert_eq!(*size, before_sync_bar_txt_manifest.size);
        p_assert_eq!(*blocksize, before_sync_bar_txt_manifest.blocksize);
        p_assert_eq!(*blocks, before_sync_bar_txt_manifest.blocks);

        let FileManifest {
            author,
            timestamp: _,
            id,
            parent,
            version,
            created: _,
            updated: _,
            size,
            blocksize: _,
            blocks,
        } = base;
        p_assert_eq!(*author, "alice@dev1".parse().unwrap());
        p_assert_eq!(*id, conflicted_id);
        p_assert_eq!(*parent, wksp1_id);
        p_assert_eq!(*version, 0);
        p_assert_eq!(*size, 0);
        p_assert_eq!(*blocks, []);
    } else if matches!(
        &local_modification,
        LocalModification::ConflictingAndNameClash
    ) {
        let wksp1_bar2_txt_id: VlobID = *env.template.get_stuff("wksp1_bar2_txt_id");

        let name_clash_id = *parent_manifest
            .children
            .get(&"bar (Parsec sync conflict 2).txt".parse().unwrap())
            .unwrap();
        p_assert_eq!(name_clash_id, wksp1_bar2_txt_id);

        let conflicted_id = *parent_manifest
            .children
            .get(&"bar (Parsec sync conflict 3).txt".parse().unwrap())
            .unwrap();
        p_assert_ne!(conflicted_id, wksp1_bar_txt_id);
        p_assert_ne!(conflicted_id, wksp1_bar2_txt_id);
        let conflicted_manifest = match wksp1_ops.store.get_manifest(conflicted_id).await.unwrap() {
            ArcLocalChildManifest::File(manifest) => manifest,
            ArcLocalChildManifest::Folder(manifest) => panic!("Expected file, got {:?}", manifest),
        };

        let LocalFileManifest {
            base,
            parent,
            need_sync,
            updated,
            size,
            blocksize,
            blocks,
        } = conflicted_manifest.as_ref();
        p_assert_eq!(*parent, before_sync_bar_txt_manifest.parent);
        p_assert_eq!(*need_sync, before_sync_bar_txt_manifest.need_sync);
        p_assert_eq!(*updated, before_sync_bar_txt_manifest.updated);
        p_assert_eq!(*size, before_sync_bar_txt_manifest.size);
        p_assert_eq!(*blocksize, before_sync_bar_txt_manifest.blocksize);
        p_assert_eq!(*blocks, before_sync_bar_txt_manifest.blocks);

        let FileManifest {
            author,
            timestamp: _,
            id,
            parent,
            version,
            created: _,
            updated: _,
            size,
            blocksize: _,
            blocks,
        } = base;
        p_assert_eq!(*author, "alice@dev1".parse().unwrap());
        p_assert_eq!(*id, conflicted_id);
        p_assert_eq!(*parent, wksp1_id);
        p_assert_eq!(*version, 0);
        p_assert_eq!(*size, 0);
        p_assert_eq!(*blocks, []);
    }

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_busy(
    #[values(
        "locked_from_the_start",
        "opened_from_the_start",
        "locked_after_remote_fetched",
        "conflict_and_parent_locked"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    env.customize(|builder| {
        builder.new_user("bob"); // bob@dev1
        builder.share_realm(wksp1_id, "bob", RealmRole::Contributor);
        builder.certificates_storage_fetch_certificates("alice@dev1");

        let bar_txt_v2_block_data = b"v2";
        let bar_txt_v2_block_access = builder
            .create_block("bob@dev1", wksp1_id, bar_txt_v2_block_data.as_ref())
            .as_block_access(0);
        builder
            .create_or_update_file_manifest_vlob("bob@dev1", wksp1_id, wksp1_bar_txt_id, None)
            .customize(|e| {
                let manifest = Arc::make_mut(&mut e.manifest);
                manifest.size = bar_txt_v2_block_data.len() as SizeInt;
                manifest.blocks = vec![bar_txt_v2_block_access];
            });

        if kind == "conflict_and_parent_locked" {
            let local_chunk_data = b"local changes";
            let local_chunk_id = builder
                .workspace_data_storage_chunk_create(
                    "alice@dev1",
                    wksp1_id,
                    local_chunk_data.as_ref(),
                )
                .map(|e| e.chunk_id);
            builder
                .workspace_data_storage_local_file_manifest_create_or_update(
                    "alice@dev1",
                    wksp1_id,
                    wksp1_bar_txt_id,
                    None,
                )
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.local_manifest);
                    let mut chunk =
                        ChunkView::new(0, (local_chunk_data.len() as SizeInt).try_into().unwrap());
                    chunk.id = local_chunk_id;
                    manifest.size = local_chunk_data.len() as SizeInt;
                    manifest.blocks = vec![vec![chunk]];
                });
        }
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await);

    let manual_entry_locks = Arc::new(Mutex::new(vec![]));
    match kind {
        "locked_from_the_start" => {
            let guard = wksp1_ops
                .store
                .test_manual_lock_entry(wksp1_bar_txt_id)
                .unwrap();
            manual_entry_locks.lock().unwrap().push(guard);
        }

        "opened_from_the_start" => {
            wksp1_ops
                .open_file_by_id(wksp1_bar_txt_id, OpenOptions::read_write())
                .await
                .unwrap();
            // The file stays open until the workspace ops is dropped as we never close it
        }

        "locked_after_remote_fetched" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // 1) Read the file manifest's vlob
                test_send_hook_vlob_read_batch!(env, wksp1_id, wksp1_bar_txt_id),
                // 2) Fetch workspace keys bundle to decrypt the vlob
                test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
            );

            let manual_entry_locks = manual_entry_locks.clone();
            let wksp1_ops = wksp1_ops.clone();
            libparsec_tests_fixtures::moment_inject_hook(
                Moment::InboundSyncRemoteFetched,
                async move {
                    let guard = wksp1_ops
                        .store
                        .test_manual_lock_entry(wksp1_bar_txt_id)
                        .unwrap();
                    manual_entry_locks.lock().unwrap().push(guard);
                },
            );
        }

        "conflict_and_parent_locked" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // 1) Read the file manifest's vlob
                test_send_hook_vlob_read_batch!(env, wksp1_id, wksp1_bar_txt_id),
                // 2) Fetch workspace keys bundle to decrypt the vlob
                test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
            );

            let manual_entry_locks = manual_entry_locks.clone();
            let wksp1_ops = wksp1_ops.clone();
            libparsec_tests_fixtures::moment_inject_hook(
                Moment::InboundSyncRemoteFetched,
                async move {
                    let guard = wksp1_ops.store.test_manual_lock_entry(wksp1_id).unwrap();
                    manual_entry_locks.lock().unwrap().push(guard);
                },
            );
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    p_assert_matches!(
        wksp1_ops.inbound_sync(wksp1_bar_txt_id).await,
        Ok(InboundSyncOutcome::EntryIsBusy)
    );

    for guard in manual_entry_locks.lock().unwrap().drain(..) {
        wksp1_ops.store.test_manual_unlock_entry(guard);
    }
}

// TODO: test inbound sync on an opened file: the sync should be rejected
// TODO: test sync with parent field changing and conflict
