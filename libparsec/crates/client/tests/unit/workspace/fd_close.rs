// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{OpenOptions, WorkspaceFdCloseError};

#[parsec_test(testbed = "minimal_client_ready")]
async fn unknown_file_descriptor(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops.fd_close(FileDescriptor(1)).await.unwrap_err();
    spy.assert_no_events();
    p_assert_matches!(err, WorkspaceFdCloseError::BadFileDescriptor);

    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn flush_needed_but_storage_stopped(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let options = OpenOptions {
        read: false,
        write: true,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops
        .open_file("/bar.txt".parse().unwrap(), options)
        .await
        .unwrap();
    ops.fd_write(fd, 0, b"hello").await.unwrap();
    ops.store.stop().await.unwrap();

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops.fd_close(fd).await.unwrap_err();
    spy.assert_no_events();
    p_assert_matches!(err, WorkspaceFdCloseError::Stopped);

    ops.stop().await.unwrap();
}
