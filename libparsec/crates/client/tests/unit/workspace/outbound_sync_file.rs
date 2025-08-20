// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_realm_get_keys_bundle,
};
use libparsec_platform_async::prelude::*;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::{
    workspace::{InboundSyncOutcome, OpenOptions, OutboundSyncOutcome},
    EventWorkspaceOpsInboundSyncDone, EventWorkspaceOpsOutboundSyncAborted,
    EventWorkspaceOpsOutboundSyncDone, EventWorkspaceOpsOutboundSyncProgress,
    EventWorkspaceOpsOutboundSyncStarted, WorkspaceOps,
};

fn assert_local_and_base_blocks_similar(manifest: &LocalFileManifest) {
    p_assert_eq!(manifest.blocks.len(), manifest.base.blocks.len());
    for (block_chunks, base_block) in manifest.blocks.iter().zip(manifest.base.blocks.iter()) {
        p_assert_eq!(block_chunks.len(), 1);

        p_assert_eq!(block_chunks[0].id, base_block.id.into());
        p_assert_eq!(block_chunks[0].start, block_chunks[0].raw_offset);
        p_assert_eq!(
            block_chunks[0].stop,
            block_chunks[0]
                .raw_size
                .checked_add(block_chunks[0].raw_offset)
                .unwrap()
        );
        p_assert_eq!(block_chunks[0].raw_offset, base_block.offset);
        p_assert_eq!(block_chunks[0].raw_size, base_block.size);
        p_assert_eq!(block_chunks[0].access.as_ref(), Some(base_block));
    }
}

async fn assert_file(
    ops: &WorkspaceOps,
    entry_id: VlobID,
    expected_need_sync: bool,
    expected_version: VersionInt,
    expected_content: &[u8],
) -> Arc<LocalFileManifest> {
    let manifest = match ops.store.get_manifest(entry_id).await.unwrap() {
        ArcLocalChildManifest::File(m) => m,
        ArcLocalChildManifest::Folder(m) => panic!("Expected file, got {m:?}"),
    };

    p_assert_eq!(manifest.need_sync, expected_need_sync);
    p_assert_eq!(manifest.base.version, expected_version);
    if !expected_need_sync {
        assert_local_and_base_blocks_similar(&manifest);
    }
    p_assert_eq!(manifest.size, expected_content.len() as SizeInt);

    let fd = ops
        .open_file_by_id(entry_id, OpenOptions::read_only())
        .await
        .unwrap();
    let mut content = Vec::with_capacity(manifest.size as usize);
    ops.fd_read(fd, 0, manifest.size, &mut content)
        .await
        .unwrap();
    p_assert_eq!(content, expected_content);
    ops.fd_close(fd).await.unwrap();

    manifest
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn non_placeholder(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    // Sanity checks
    assert_file(&wksp1_ops, wksp1_bar_txt_id, false, 1, b"hello world").await;

    const NEW_DATA: &[u8] = b"new data";

    // Do a change requiring a sync
    let options = OpenOptions {
        read: false,
        write: true,
        truncate: true,
        create: false,
        create_new: false,
    };
    let fd = wksp1_ops
        .open_file_by_id(wksp1_bar_txt_id, options)
        .await
        .unwrap();
    wksp1_ops.fd_write(fd, 0, NEW_DATA).await.unwrap();
    wksp1_ops.fd_close(fd).await.unwrap();

    // Check the workspace manifest is need sync
    assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 1, NEW_DATA).await;

    // Mock server commands and do the sync

    let expected_base_blocks = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let (key_derivation, key_index) = env.get_last_realm_key(wksp1_id);
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch last workspace keys bundle to encrypt the new manifest
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 3) `block_create`
        {
            let expected_base_blocks = expected_base_blocks.clone();
            let key_derivation = key_derivation.to_owned();
            move |req: authenticated_cmds::latest::block_create::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, key_index);
                let key = key_derivation.derive_secret_key_from_uuid(*req.block_id);
                p_assert_eq!(key.decrypt(&req.block).unwrap(), NEW_DATA);
                expected_base_blocks.lock().unwrap().push(BlockAccess {
                    id: req.block_id,
                    offset: 0,
                    size: (NEW_DATA.len() as u64).try_into().unwrap(),
                    digest: HashDigest::from_data(NEW_DATA),
                });
                authenticated_cmds::latest::block_create::Rep::Ok {}
            }
        },
        // 2) `vlob_update` succeed on first try !
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.key_index, key_index);
            p_assert_eq!(req.vlob_id, wksp1_bar_txt_id);
            p_assert_eq!(req.version, 2);
            authenticated_cmds::latest::vlob_update::Rep::Ok {}
        },
    );

    let mut spy = wksp1_ops.event_bus.spy.start_expecting();

    let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::Done);
    spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
        p_assert_eq!(event.realm_id, wksp1_id);
        p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
    });
    spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncProgress| {
        p_assert_eq!(event.realm_id, wksp1_id);
        p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        p_assert_eq!(event.block_index, 0);
        p_assert_eq!(event.blocks, 1);
        p_assert_eq!(event.blocksize, 512);
    });
    spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncDone| {
        p_assert_eq!(event.realm_id, wksp1_id);
        p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
    });

    // Check the user manifest is not longer need sync
    let foo_manifest = assert_file(&wksp1_ops, wksp1_bar_txt_id, false, 2, NEW_DATA).await;
    p_assert_eq!(
        foo_manifest.base.blocks,
        *expected_base_blocks.lock().unwrap()
    );

    // Subsequent sync is an idempotent noop

    let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::Done);
    spy.assert_no_events();

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn inbound_sync_needed(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    env.customize(|builder| {
        // New version of the file that we our client doesn't know about
        builder.create_or_update_file_manifest_vlob("alice@dev1", wksp1_id, wksp1_bar_txt_id, None);
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    // Truncate the file to require a sync

    let options = OpenOptions {
        read: false,
        write: true,
        truncate: true,
        create: false,
        create_new: false,
    };
    let fd = wksp1_ops
        .open_file_by_id(wksp1_bar_txt_id, options)
        .await
        .unwrap();
    wksp1_ops.fd_close(fd).await.unwrap();

    // Mock server commands and do the sync

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch last workspace keys bundle to encrypt the new manifest
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // No `block_create` as we only truncated the file
        // 2) `vlob_update`
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.vlob_id, wksp1_bar_txt_id);
            p_assert_eq!(req.version, 2);
            authenticated_cmds::latest::vlob_update::Rep::BadVlobVersion
        },
    );

    // Sanity check
    let before_sync_manifest = assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 1, b"").await;

    let mut spy = wksp1_ops.event_bus.spy.start_expecting();

    let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::InboundSyncNeeded);
    spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
        p_assert_eq!(event.realm_id, wksp1_id);
        p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
    });
    spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncAborted| {
        p_assert_eq!(event.realm_id, wksp1_id);
        p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
    });

    // Check the user manifest still need sync
    let after_failed_sync_manifest = assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 1, b"").await;
    p_assert_eq!(after_failed_sync_manifest, before_sync_manifest);

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn entry_is_busy_after_local_retrieved(
    #[values(false, true)] concurrent_write: bool,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_ops = Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await);

    // 0) Do a change requiring a sync

    const NEW_DATA: &[u8] = b"new data";

    let options = OpenOptions {
        read: false,
        write: true,
        truncate: true,
        create: false,
        create_new: false,
    };
    let fd = wksp1_ops
        .open_file_by_id(wksp1_bar_txt_id, options)
        .await
        .unwrap();
    wksp1_ops.fd_write(fd, 0, NEW_DATA).await.unwrap();
    wksp1_ops.fd_close(fd).await.unwrap();

    // Sanity check
    assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 1, NEW_DATA).await;

    // 1) Configure the hook so that the file gets opened while synchronization
    //    has started

    let concurrent_file_lock_requested = event::Event::<()>::new();
    let concurrent_file_unlock_requested = event::Event::<()>::new();
    let (concurrent_file_locked, concurrent_file_unlocked) = {
        let lock_requested = concurrent_file_lock_requested.listen();
        let unlock_requested = concurrent_file_unlock_requested.listen();
        let locked = event::Event::<()>::new();
        let on_locked = locked.listen();
        let unlocked = event::Event::<()>::new();
        let on_unlocked = unlocked.listen();
        let wksp1_ops = wksp1_ops.clone();

        spawn(async move {
            lock_requested.await;
            let fd = wksp1_ops
                .open_file_by_id(wksp1_bar_txt_id, OpenOptions::read_write())
                .await
                .unwrap();

            if concurrent_write {
                wksp1_ops.fd_write(fd, 4, b"troubles").await.unwrap();
            }

            locked.notify(1);
            unlock_requested.await;

            wksp1_ops.fd_close(fd).await.unwrap();

            unlocked.notify(1);
        });

        (on_locked, on_unlocked)
    };

    libparsec_tests_fixtures::moment_inject_hook(Moment::OutboundSyncLocalRetrieved, async move {
        concurrent_file_lock_requested.notify(1);
        concurrent_file_locked.await;
    });

    let expected_content = if concurrent_write {
        b"new troubles"
    } else {
        NEW_DATA
    };

    // 2) The synchronization couldn't successfully finish

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
        p_assert_matches!(outcome, OutboundSyncOutcome::EntryIsBusy);

        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
        // The `NEW_DATA` block is being uploaded
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncProgress| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
            p_assert_eq!(event.block_index, 0);
            p_assert_eq!(event.blocks, 1);
            p_assert_eq!(event.blocksize, 512);
        });
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncAborted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
    }

    // The outbound sync has failed. Since we injected the concurrent open at
    // `Moment::OutboundSyncLocalRetrieved` and the file was already reshaped,
    // the corresponding block and manifest have been uploaded to the server.
    // However, the manifest could not be updated locally to acknowledge the
    // new remote version.

    // 3) The file is still opened, this time the sync cannot even start

    {
        let spy = wksp1_ops.event_bus.spy.start_expecting();
        let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
        p_assert_matches!(outcome, OutboundSyncOutcome::EntryIsBusy);
        // The abort is so early there is no events !
        spy.assert_no_events();
    }

    // 4) Now close the file, the next outbound sync should fail since the manifest
    // has already been uploaded

    {
        concurrent_file_unlock_requested.notify(1);
        concurrent_file_unlocked.await;
        assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 1, expected_content).await;

        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
        p_assert_matches!(outcome, OutboundSyncOutcome::InboundSyncNeeded);

        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
        // The new `new_troubles` block is being uploaded
        if concurrent_write {
            spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncProgress| {
                p_assert_eq!(event.realm_id, wksp1_id);
                p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
                p_assert_eq!(event.block_index, 0);
                p_assert_eq!(event.blocks, 1);
                p_assert_eq!(event.blocksize, 512);
            });
        }
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncAborted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
    }

    // 5) Perform the inbound sync

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        let outcome = wksp1_ops.inbound_sync(wksp1_bar_txt_id).await.unwrap();
        p_assert_matches!(outcome, InboundSyncOutcome::Updated);

        // Version 2 is acknowledged
        assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 2, expected_content).await;

        spy.assert_next(|event: &EventWorkspaceOpsInboundSyncDone| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
    }

    // 6) Now the next outbound sync should work

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
        p_assert_matches!(outcome, OutboundSyncOutcome::Done);

        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncDone| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
    }

    // Check the manifest is no longer need sync, and bumped to version 3
    assert_file(&wksp1_ops, wksp1_bar_txt_id, false, 3, expected_content).await;

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn entry_is_busy_after_file_reshaped_and_block_uploaded(
    #[values(false, true)] concurrent_write: bool,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_ops = Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await);

    // 0) Do a change requiring a sync

    const NEW_DATA: &[u8] = b"new data";

    let options = OpenOptions {
        read: false,
        write: true,
        truncate: true,
        create: false,
        create_new: false,
    };
    let fd = wksp1_ops
        .open_file_by_id(wksp1_bar_txt_id, options)
        .await
        .unwrap();
    wksp1_ops.fd_write(fd, 0, NEW_DATA).await.unwrap();
    wksp1_ops.fd_close(fd).await.unwrap();

    // Sanity check
    assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 1, NEW_DATA).await;

    // 1) Configure the hook so that the file gets opened while synchronization
    //    has started

    let concurrent_file_lock_requested = event::Event::<()>::new();
    let concurrent_file_unlock_requested = event::Event::<()>::new();
    let (concurrent_file_locked, concurrent_file_unlocked) = {
        let lock_requested = concurrent_file_lock_requested.listen();
        let unlock_requested = concurrent_file_unlock_requested.listen();
        let locked = event::Event::<()>::new();
        let on_locked = locked.listen();
        let unlocked = event::Event::<()>::new();
        let on_unlocked = unlocked.listen();
        let wksp1_ops = wksp1_ops.clone();

        spawn(async move {
            lock_requested.await;
            let fd = wksp1_ops
                .open_file_by_id(wksp1_bar_txt_id, OpenOptions::read_write())
                .await
                .unwrap();

            if concurrent_write {
                wksp1_ops.fd_write(fd, 4, b"troubles").await.unwrap();
            }

            locked.notify(1);
            unlock_requested.await;

            wksp1_ops.fd_close(fd).await.unwrap();

            unlocked.notify(1);
        });

        (on_locked, on_unlocked)
    };

    libparsec_tests_fixtures::moment_inject_hook(
        Moment::OutboundSyncFileReshapedAndBlockUploaded,
        async move {
            concurrent_file_lock_requested.notify(1);
            concurrent_file_locked.await;
        },
    );

    let expected_content = if concurrent_write {
        b"new troubles"
    } else {
        NEW_DATA
    };

    // 2) The synchronization couldn't successfully finish

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
        p_assert_matches!(outcome, OutboundSyncOutcome::EntryIsBusy);

        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncProgress| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
            p_assert_eq!(event.block_index, 0);
            p_assert_eq!(event.blocks, 1);
            p_assert_eq!(event.blocksize, 512);
        });
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncAborted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
    }

    // The outbound sync has failed. Since we injected the concurrent open at
    // `Moment::OutboundSyncFileReshapedAndBlockUploaded` the failure occurred
    // after server blocks & manifest upload (the manifest could not be updated
    // on local to acknowledge the new remote version).

    // 3) The file is still opened, this time the sync cannot even start

    {
        let spy = wksp1_ops.event_bus.spy.start_expecting();
        let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
        p_assert_matches!(outcome, OutboundSyncOutcome::EntryIsBusy);
        // The abort is so early there is no events !
        spy.assert_no_events();
    }

    // 4) Now close the file, the next sync should proceed

    {
        concurrent_file_unlock_requested.notify(1);
        concurrent_file_unlocked.await;
        assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 1, expected_content).await;

        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();

        // This sync will clash with the previously uploaded data
        p_assert_matches!(outcome, OutboundSyncOutcome::InboundSyncNeeded);

        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
        if concurrent_write {
            spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncProgress| {
                p_assert_eq!(event.realm_id, wksp1_id);
                p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
                p_assert_eq!(event.block_index, 0);
                p_assert_eq!(event.blocks, 1);
                p_assert_eq!(event.blocksize, 512);
            });
        }
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncAborted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
    }

    // Check the manifest is still need sync
    assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 1, expected_content).await;

    // 5) Bonus check: ensure doing an inbound sync will correctly take into account
    //    the previously uploaded data

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        wksp1_ops.inbound_sync(wksp1_bar_txt_id).await.unwrap();

        if concurrent_write {
            assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 2, expected_content).await;
        } else {
            // TODO: currently inbound sync's remote/local merge doesn't recompute
            // need sync when remote manifest author is self !
            assert_file(&wksp1_ops, wksp1_bar_txt_id, true, 2, expected_content).await;
        }

        spy.assert_next(|event: &EventWorkspaceOpsInboundSyncDone| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
    }

    // 6) Bonus check: now the final outbound sync will succeed !

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();

        assert_file(&wksp1_ops, wksp1_bar_txt_id, false, 3, expected_content).await;

        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncDone| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_bar_txt_id);
        });
    }

    wksp1_ops.stop().await.unwrap();
}

// TODO: test with placeholder folder manifest
// TODO: test sync with parent field changing and conflict
