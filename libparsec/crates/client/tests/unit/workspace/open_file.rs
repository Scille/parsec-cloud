// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{assert_ls, ls, workspace_ops_factory};
use crate::{
    workspace::{EntryStat, OpenOptions, WorkspaceOpenFileError},
    EventWorkspaceOpsOutboundSyncNeeded,
};

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_existing(#[values(true, false)] open_in_root: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

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
                append: false,
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

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let path = if open_in_root {
        "/foo".parse().unwrap()
    } else {
        "/foo/spam".parse().unwrap()
    };

    let err = ops
        .open_file(
            path,
            OpenOptions {
                read: false,
                write: false,
                append: false,
                truncate: false,
                create: false,
                create_new: false,
            },
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceOpenFileError::EntryNotAFile);
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn cannot_open_root(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops
        .open_file(
            "/".parse().unwrap(),
            OpenOptions {
                read: false,
                write: false,
                append: false,
                truncate: false,
                create: false,
                create_new: false,
            },
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, WorkspaceOpenFileError::EntryNotAFile);
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn open_with_create(#[values(true, false)] file_already_exists: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;
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
        append: false,
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
            EntryStat::File { id, .. } => id,
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
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;
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
        append: false,
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
            EntryStat::File { id, .. } => id,
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
async fn multiple_opens(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let path: FsPath = "/bar.txt".parse().unwrap();

    let options = OpenOptions {
        read: false,
        write: false,
        append: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd1 = ops.open_file(path.clone(), options).await.unwrap();
    p_assert_eq!(fd1.0, 1);

    // Can use different options for the second open
    let options = OpenOptions {
        read: true,
        write: true,
        append: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd2 = ops.open_file(path, options).await.unwrap();
    p_assert_eq!(fd2.0, 2);

    spy.assert_no_events();

    ops.fd_close(fd1).await.unwrap();
    ops.fd_close(fd2).await.unwrap();
}

// TODO: open with parent or target manifest not in client
// TODO: open_with_truncate once read is available !
// TODO: open_with_append once read&write are available !
// TODO: open for read/write/read&write once read&write are available !
// TODO: multiple_opens_with_truncate once read is available !
// TODO: truncate on open also remove the chunks in the manifest and in the local storage
