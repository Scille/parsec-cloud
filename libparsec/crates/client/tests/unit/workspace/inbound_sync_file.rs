// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;

enum RemoteModification {
    Nothing,
    Overwritten,
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
    use std::sync::Arc;

    if matches!(
        (&local_modification, &remote_modification),
        (
            LocalModification::Conflicting | LocalModification::ConflictingAndNameClash,
            RemoteModification::Nothing
        )
    ) {
        // Meaningless case, just skip it
        return;
    }

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    // 1) Customize testbed

    let env = env.customize(|builder| {
        builder.new_device("alice"); // alice@dev2
        builder.certificates_storage_fetch_certificates("alice@dev1");

        match remote_modification {
            RemoteModification::Nothing => (),
            RemoteModification::Overwritten => {
                let block_data = b"abc";
                let (wksp1_bar_txt_new_block_id, wksp1_bar_txt_new_block_key_index) = builder
                    .create_block("alice@dev2", wksp1_id, block_data.as_ref())
                    .map(|e| (e.block_id, e.key_index));
                builder.store_stuff("wksp1_bar_txt_new_block_id", &wksp1_bar_txt_new_block_id);
                builder
                    .create_or_update_file_manifest_vlob("alice@dev2", wksp1_id, wksp1_bar_txt_id)
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.manifest);
                        manifest.blocks.clear();
                        manifest.size = block_data.len() as u64;
                        manifest.blocks.push(BlockAccess {
                            id: wksp1_bar_txt_new_block_id,
                            key_index: wksp1_bar_txt_new_block_key_index,
                            key: None,
                            offset: 0,
                            size: manifest.size.try_into().unwrap(),
                            digest: HashDigest::from_data(block_data),
                        });
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
                        Some(wksp1_bar_txt_id),
                    )
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.local_manifest);
                        manifest.need_sync = true;
                        manifest.blocks.clear();
                        manifest.size = 3;
                        manifest.blocks.push(vec![Chunk {
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
                            manifest
                                .children
                                .insert("bar (2).txt".parse().unwrap(), wksp1_bar2_txt_id);
                        });
                }
            }
            (
                LocalModification::Conflicting | LocalModification::ConflictingAndNameClash,
                RemoteModification::Nothing,
            ) => {
                unreachable!()
            }
        }
    });

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
        .get_child_manifest(wksp1_bar_txt_id)
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
                        wksp1_bar_txt_last_remote_manifest.author.clone(),
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
        {
            let keys_bundle = env.get_last_realm_keys_bundle(wksp1_id);
            let keys_bundle_access =
                env.get_last_realm_keys_bundle_access_for(wksp1_id, alice.user_id());
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
    );

    wksp1_ops.inbound_sync(wksp1_bar_txt_id).await.unwrap();
    let bar_txt_manifest = match wksp1_ops
        .store
        .get_child_manifest(wksp1_bar_txt_id)
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

    let parent_manifest = wksp1_ops.store.get_workspace_manifest();
    p_assert_eq!(
        *parent_manifest
            .children
            .get(&"bar.txt".parse().unwrap())
            .unwrap(),
        wksp1_bar_txt_id,
    );

    if matches!(&local_modification, LocalModification::Conflicting) {
        let conflicted_id = *parent_manifest
            .children
            .get(&"bar (2).txt".parse().unwrap())
            .unwrap();
        p_assert_ne!(conflicted_id, wksp1_bar_txt_id);
        let conflicted_manifest = match wksp1_ops
            .store
            .get_child_manifest(conflicted_id)
            .await
            .unwrap()
        {
            ArcLocalChildManifest::File(manifest) => manifest,
            ArcLocalChildManifest::Folder(manifest) => panic!("Expected file, got {:?}", manifest),
        };

        let LocalFileManifest {
            base,
            need_sync,
            updated,
            size,
            blocksize,
            blocks,
        } = conflicted_manifest.as_ref();
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
            .get(&"bar (2).txt".parse().unwrap())
            .unwrap();
        p_assert_eq!(name_clash_id, wksp1_bar2_txt_id);

        let conflicted_id = *parent_manifest
            .children
            .get(&"bar (3).txt".parse().unwrap())
            .unwrap();
        p_assert_ne!(conflicted_id, wksp1_bar_txt_id);
        p_assert_ne!(conflicted_id, wksp1_bar2_txt_id);
        let conflicted_manifest = match wksp1_ops
            .store
            .get_child_manifest(conflicted_id)
            .await
            .unwrap()
        {
            ArcLocalChildManifest::File(manifest) => manifest,
            ArcLocalChildManifest::Folder(manifest) => panic!("Expected file, got {:?}", manifest),
        };

        let LocalFileManifest {
            base,
            need_sync,
            updated,
            size,
            blocksize,
            blocks,
        } = conflicted_manifest.as_ref();
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

// TODO: test inbound sync on an opened file: the sync should be rejected
