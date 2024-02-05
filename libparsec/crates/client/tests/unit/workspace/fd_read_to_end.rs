// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{OpenOptions, WorkspaceFdReadToEndError};

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let path = "/bar.txt".parse().unwrap();
    let options = OpenOptions {
        read: true,
        write: false,
        append: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops.open_file(path, options).await.unwrap();
    p_assert_eq!(fd.0, 1);

    let mut buf = vec![];

    let read_bytes = ops.fd_read_to_end(fd, &mut buf).await.unwrap();
    spy.assert_no_events();

    let expected_buf = b"hello world";
    p_assert_eq!(read_bytes, expected_buf.len());
    p_assert_eq!(buf, expected_buf);

    ops.fd_close(fd).await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn cursor_not_in_read_mode(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let path = "/bar.txt".parse().unwrap();
    let options = OpenOptions {
        read: false,
        write: false,
        append: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops.open_file(path, options).await.unwrap();
    p_assert_eq!(fd.0, 1);

    let mut buf = vec![];

    let err = ops.fd_read_to_end(fd, &mut buf).await.unwrap_err();
    spy.assert_no_events();
    // TODO: bad error type
    p_assert_matches!(err, WorkspaceFdReadToEndError::NotInReadMode);

    ops.fd_close(fd).await.unwrap();
}

// TODO: test invalid manifest with less block that in declared size
