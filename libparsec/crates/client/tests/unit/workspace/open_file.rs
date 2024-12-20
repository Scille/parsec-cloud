// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    num::NonZeroU64,
    sync::{Arc, Mutex},
};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls, ls, workspace_ops_factory};
use crate::{
    workspace::{
        EntryStat, FileStat, OpenOptions, WorkspaceFdReadError, WorkspaceFdWriteError,
        WorkspaceOpenFileError,
    },
    EventWorkspaceOpsOutboundSyncNeeded,
};

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_existing(#[values(true, false)] open_in_root: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let spy = ops.event_bus.spy.start_expecting();

    let path = if open_in_root {
        "/dummy.txt".parse().unwrap()
    } else {
        "/foo/dummy.txt".parse().unwrap()
    };

    let err = ops
        .open_file(
            path,
            OpenOptions {
                read: false,
                write: false,
                truncate: false,
                create: false,
                create_new: false,
            },
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceOpenFileError::EntryNotFound);
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn cannot_open_folder(#[values(true, false)] open_in_root: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let spy = ops.event_bus.spy.start_expecting();

    let (path, expected_entry_id) = if open_in_root {
        ("/foo".parse().unwrap(), wksp1_foo_id)
    } else {
        ("/foo/spam".parse().unwrap(), wksp1_foo_spam_id)
    };

    let err = ops
        .open_file(
            path,
            OpenOptions {
                read: false,
                write: false,
                truncate: false,
                create: false,
                create_new: false,
            },
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceOpenFileError::EntryNotAFile { entry_id } if entry_id == expected_entry_id);
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn cannot_open_root(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops
        .open_file(
            "/".parse().unwrap(),
            OpenOptions {
                read: false,
                write: false,
                truncate: false,
                create: false,
                create_new: false,
            },
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceOpenFileError::EntryNotAFile { entry_id} if entry_id == wksp1_id);
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn open_for_read_write(
    #[values(true, false)] read_mode: bool,
    #[values(true, false)] write_mode: bool,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    let options = OpenOptions {
        read: read_mode,
        write: write_mode,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops
        .open_file("/bar.txt".parse().unwrap(), options)
        .await
        .unwrap();

    let mut buff = vec![];
    let outcome = ops.fd_read(fd, 5, 5, &mut buff).await;
    if read_mode {
        p_assert_matches!(outcome, Ok(5));
        // cspell:disable-next-line
        p_assert_eq!(buff, b" worl");
    } else {
        p_assert_matches!(outcome, Err(WorkspaceFdReadError::NotInReadMode));
        assert!(buff.is_empty());
    }

    let outcome = ops.fd_write(fd, 5, b"foo").await;
    if write_mode {
        p_assert_matches!(outcome, Ok(3));
    } else {
        p_assert_matches!(outcome, Err(WorkspaceFdWriteError::NotInWriteMode));
    }

    ops.fd_close(fd).await.unwrap();
    if write_mode {
        spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
            p_assert_eq!(e.realm_id, wksp1_id);
            p_assert_eq!(e.entry_id, wksp1_bar_txt_id);
        });
    } else {
        spy.assert_no_events();
    }

    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn open_with_create(#[values(true, false)] file_already_exists: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;
    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    let path = if file_already_exists {
        "/bar.txt".parse().unwrap()
    } else {
        "/new_file.txt".parse().unwrap()
    };

    let options = OpenOptions {
        read: false,
        write: false,
        truncate: false,
        create: true,
        create_new: false,
    };
    let fd = ops.open_file(path, options).await.unwrap();
    p_assert_matches!(fd.0, 1);

    if file_already_exists {
        assert_ls!(ops, "/", ["bar.txt", "foo"]).await;
        spy.assert_no_events();
    } else {
        assert_ls!(ops, "/", ["bar.txt", "foo", "new_file.txt"]).await;
        let stat = ops
            .stat_entry(&"/new_file.txt".parse().unwrap())
            .await
            .unwrap();
        let new_file_id = match stat {
            EntryStat::File { base, .. } => base.id,
            EntryStat::Folder { .. } => unreachable!(),
        };
        spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
            p_assert_eq!(e.realm_id, wksp1_id);
            p_assert_eq!(e.entry_id, new_file_id);
        });
        spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
            p_assert_eq!(e.realm_id, wksp1_id);
            p_assert_eq!(e.entry_id, wksp1_id);
        });
    }

    ops.fd_close(fd).await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn open_with_create_new(#[values(true, false)] file_already_exists: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;
    assert_ls!(ops, "/", ["bar.txt", "foo"]).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    let path = if file_already_exists {
        "/bar.txt".parse().unwrap()
    } else {
        "/new_file.txt".parse().unwrap()
    };

    let options = OpenOptions {
        read: false,
        write: false,
        truncate: false,
        create: false,
        create_new: true,
    };
    let outcome = ops.open_file(path, options).await;

    if file_already_exists {
        let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
        p_assert_matches!(outcome, Err(WorkspaceOpenFileError::EntryExistsInCreateNewMode { entry_id }) if entry_id == wksp1_bar_txt_id);
        assert_ls!(ops, "/", ["bar.txt", "foo"]).await;
        spy.assert_no_events();
    } else {
        match outcome {
            Ok(fd) => {
                ops.fd_close(fd).await.unwrap();
            }
            err => unreachable!("Unexpected outcome: {:?}", err),
        }

        assert_ls!(ops, "/", ["bar.txt", "foo", "new_file.txt"]).await;
        let stat = ops
            .stat_entry(&"/new_file.txt".parse().unwrap())
            .await
            .unwrap();
        let new_file_id = match stat {
            EntryStat::File { base, .. } => base.id,
            EntryStat::Folder { .. } => unreachable!(),
        };
        spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
            p_assert_eq!(e.realm_id, wksp1_id);
            p_assert_eq!(e.entry_id, new_file_id);
        });
        spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
            p_assert_eq!(e.realm_id, wksp1_id);
            p_assert_eq!(e.entry_id, wksp1_id);
        });
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn open_with_truncate(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    let options = OpenOptions {
        read: false,
        write: true,
        truncate: true,
        create: false,
        create_new: false,
    };
    let fd = ops
        .open_file("/bar.txt".parse().unwrap(), options)
        .await
        .unwrap();

    ops.fd_close(fd).await.unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_bar_txt_id);
    });

    match ops.stat_entry_by_id(wksp1_bar_txt_id).await.unwrap() {
        EntryStat::File {
            base:
                FileStat {
                    base_version,
                    is_placeholder,
                    need_sync,
                    size,
                    ..
                },
            ..
        } => {
            p_assert_eq!(need_sync, true);
            p_assert_eq!(size, 0);
            p_assert_eq!(is_placeholder, false);
            p_assert_eq!(base_version, 1);
        }
        bad @ EntryStat::Folder { .. } => panic!("Expected file, got {bad:?}"),
    }

    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn multiple_opens(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    let path: FsPath = "/bar.txt".parse().unwrap();

    let fd1_options = OpenOptions {
        read: true,
        write: true,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd1 = ops.open_file(path.clone(), fd1_options).await.unwrap();
    p_assert_eq!(fd1.0, 1);

    // Can use different options for the second open
    let fd2_options = OpenOptions {
        read: true,
        write: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd2 = ops.open_file(path, fd2_options).await.unwrap();
    p_assert_eq!(fd2.0, 2);

    spy.assert_no_events();

    // Both file descriptor see the same data
    ops.fd_write(fd1, 6, b"from the other side").await.unwrap();
    let mut fd2_buff = vec![];
    ops.fd_read(fd2, 0, 100, &mut fd2_buff).await.unwrap();
    p_assert_eq!(fd2_buff, b"hello from the other side");

    ops.fd_close(fd1).await.unwrap();
    ops.fd_close(fd2).await.unwrap();

    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_bar_txt_id);
    });
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn write_open_but_no_modif(
    #[values(
        "synced_non_empty_then_open_for_write",
        "synced_non_empty_then_open_for_create",
        "synced_empty_then_open_for_write",
        "synced_empty_then_truncate_on_open",
        "need_sync_empty_then_truncate_on_open",
        "need_sync_empty_then_open_for_write",
        "need_sync_empty_then_open_for_create",
        "need_sync_non_empty_then_open_for_write"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let mut open_options = OpenOptions {
        read: false,
        write: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let mut customize_synced_empty = false;
    let mut customize_need_sync_empty = false;
    let mut customize_need_sync_non_empty = false;
    match kind {
        "synced_non_empty_then_open_for_write" => {
            open_options.write = true;
        }
        "synced_non_empty_then_open_for_create" => {
            open_options.write = true;
            open_options.create = true;
        }
        "synced_empty_then_open_for_write" => {
            customize_synced_empty = true;
            open_options.write = true;
        }
        "synced_empty_then_truncate_on_open" => {
            customize_synced_empty = true;
            open_options.write = true;
            open_options.truncate = true;
        }
        "need_sync_empty_then_truncate_on_open" => {
            customize_need_sync_empty = true;
            open_options.write = true;
            open_options.truncate = true;
        }
        "need_sync_empty_then_open_for_write" => {
            customize_need_sync_empty = true;
            open_options.write = true;
        }
        "need_sync_empty_then_open_for_create" => {
            open_options.write = true;
            open_options.create = true;
        }
        "need_sync_non_empty_then_open_for_write" => {
            customize_need_sync_non_empty = true;
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    if customize_synced_empty {
        env.customize(|builder| {
            builder
                .create_or_update_file_manifest_vlob("alice@dev1", wksp1_id, wksp1_bar_txt_id, None)
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.manifest);
                    manifest.blocks.clear();
                    manifest.size = 0;
                });
            builder.workspace_data_storage_fetch_file_vlob(
                "alice@dev1",
                wksp1_id,
                wksp1_bar_txt_id,
            );
        })
        .await;
    }
    if customize_need_sync_empty {
        env.customize(|builder| {
            builder
                .workspace_data_storage_local_file_manifest_create_or_update(
                    "alice@dev1",
                    wksp1_id,
                    wksp1_bar_txt_id,
                    None,
                )
                .customize(|e| {
                    let local_manifest = Arc::make_mut(&mut e.local_manifest);
                    local_manifest.blocks.clear();
                    local_manifest.size = 0;
                    local_manifest.need_sync = true;
                });
        })
        .await;
    }
    if customize_need_sync_non_empty {
        env.customize(|builder| {
            builder
                .workspace_data_storage_local_file_manifest_create_or_update(
                    "alice@dev1",
                    wksp1_id,
                    wksp1_bar_txt_id,
                    None,
                )
                .customize(|e| {
                    let local_manifest = Arc::make_mut(&mut e.local_manifest);
                    local_manifest.blocks[0][0].stop = NonZeroU64::new(1).unwrap();
                    local_manifest.size = 1;
                    local_manifest.need_sync = true;
                });
        })
        .await;
    }

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let spy = ops.event_bus.spy.start_expecting();

    let fd = ops
        .open_file("/bar.txt".parse().unwrap(), open_options)
        .await
        .unwrap();

    ops.fd_close(fd).await.unwrap();
    spy.assert_no_events();

    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn close_on_workspace_ops_stop(
    #[values(false, true)] with_concurrent_open: bool,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");

    let alice = env.local_device("alice@dev1");

    let ops = Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await);

    let fd1 = ops
        .open_file_by_id(wksp1_bar_txt_id, OpenOptions::read_write())
        .await
        .unwrap();
    let outcome = ops.fd_write(fd1, 6, b"after the close").await;
    p_assert_matches!(outcome, Ok(15));

    let _fd2 = ops
        .open_file_by_id(wksp1_foo_egg_txt_id, OpenOptions::read_only())
        .await
        .unwrap();

    let concurrent_open_outcome = Arc::new(Mutex::new(None));
    if with_concurrent_open {
        let ops = ops.clone();
        let concurrent_open_outcome = concurrent_open_outcome.clone();

        libparsec_tests_fixtures::moment_inject_hook(
            Moment::WorkspaceOpsStopAllFdsClosed,
            async move {
                let outcome = ops
                    .open_file_by_id(wksp1_bar_txt_id, OpenOptions::read_only())
                    .await;
                concurrent_open_outcome
                    .lock()
                    .expect("Mutex is poisoned")
                    .replace(outcome);
            },
        );
    }

    // Stop should close the files

    let config = ops.config.clone();
    let device = ops.device.clone();
    let cmds = ops.cmds.clone();
    let certificates_ops = ops.certificates_ops.clone();
    let realm_id = ops.realm_id;
    let event_bus = crate::EventBus::default();

    ops.stop().await.unwrap();

    let ops = crate::workspace::WorkspaceOps::start(
        config,
        device,
        cmds,
        certificates_ops,
        event_bus,
        realm_id,
        crate::workspace::WorkspaceExternalInfo {
            entry: LocalUserManifestWorkspaceEntry {
                id: realm_id,
                name: "wksp1".parse().unwrap(),
                name_origin: CertificateBasedInfoOrigin::Placeholder,
                role: RealmRole::Owner,
                role_origin: CertificateBasedInfoOrigin::Placeholder,
            },
            workspace_index: 0,
            total_workspaces: 1,
        },
    )
    .await
    .unwrap();

    if with_concurrent_open {
        let concurrent_open_outcome = concurrent_open_outcome
            .lock()
            .expect("Mutex is poisoned")
            .take()
            .expect("Concurrent open not called !");
        p_assert_matches!(
            concurrent_open_outcome,
            Err(WorkspaceOpenFileError::Stopped)
        );
    }

    // Restart to ensure the file has been correctly modified

    let fd3 = ops
        .open_file_by_id(wksp1_bar_txt_id, OpenOptions::read_only())
        .await
        .unwrap();
    let mut fd3_buff = vec![];
    ops.fd_read(fd3, 0, 100, &mut fd3_buff).await.unwrap();
    p_assert_eq!(fd3_buff, b"hello after the close");
}

// TODO: open with parent or target manifest not in client
// TODO: multiple_opens_with_truncate once read is available !
// TODO: truncate on open also remove the chunks in the manifest and in the local storage
