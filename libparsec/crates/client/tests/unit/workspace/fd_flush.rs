// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: test write buffer under constrained_io

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::{
    workspace::{EntryStat, OpenOptions, WorkspaceFdFlushError},
    EventWorkspaceOpsOutboundSyncNeeded,
};

#[parsec_test(testbed = "minimal_client_ready")]
async fn write_then_flush(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // 1) Open the file and do the write & flush

    let initial_size = match ops.stat_entry_by_id(wksp1_bar_txt_id).await.unwrap() {
        EntryStat::File { base, .. } => base.size,
        EntryStat::Folder { .. } => unreachable!(),
    };

    let options = OpenOptions {
        read: false,
        write: true,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops
        .open_file_by_id(wksp1_bar_txt_id, options)
        .await
        .unwrap();

    let mut spy = ops.event_bus.spy.start_expecting();

    ops.fd_write_start_eof(fd, b"new").await.unwrap();
    ops.fd_flush(fd).await.unwrap();

    // Flush makes metadata visible
    let size = match ops.stat_entry_by_id(wksp1_bar_txt_id).await.unwrap() {
        EntryStat::File { base, .. } => base.size,
        EntryStat::Folder { .. } => unreachable!(),
    };
    p_assert_eq!(size, initial_size + 3);

    // Additional flush is a noop
    ops.fd_flush(fd).await.unwrap();

    spy.assert_no_events(); // Event is only triggered on file close

    ops.fd_close(fd).await.unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_bar_txt_id);
    });

    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_change_then_flush(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // 1) Open the file and do the flush

    let options = OpenOptions {
        read: false,
        write: true,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops
        .open_file_by_id(wksp1_bar_txt_id, options)
        .await
        .unwrap();

    let spy = ops.event_bus.spy.start_expecting();

    ops.fd_flush(fd).await.unwrap();

    ops.fd_close(fd).await.unwrap();
    spy.assert_no_events(); // Flush was a noop

    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn cursor_not_in_write_mode(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let options = OpenOptions {
        read: false,
        write: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops
        .open_file_by_id(wksp1_bar_txt_id, options)
        .await
        .unwrap();
    p_assert_eq!(fd.0, 1);

    let err = ops.fd_flush(fd).await.unwrap_err();
    spy.assert_no_events();
    p_assert_matches!(err, WorkspaceFdFlushError::NotInWriteMode);

    ops.fd_close(fd).await.unwrap();
    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn unknown_file_descriptor(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops.fd_flush(FileDescriptor(1)).await.unwrap_err();
    spy.assert_no_events();
    p_assert_matches!(err, WorkspaceFdFlushError::BadFileDescriptor);

    ops.stop().await.unwrap();
}
