// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: test write buffer under constrained_io
// TODO: test write buffer of zero length
// TODO: test write buffer of zero length under constrained_io
// TODO: test write buffer of zero length under constrained_io and cursor at the end of the file

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{restart_workspace_ops, workspace_ops_factory};
use crate::{
    workspace::{OpenOptions, WorkspaceFdWriteError},
    WorkspaceOps,
};

async fn open_for_read(ops: &WorkspaceOps, path: &str) -> FileDescriptor {
    let options = OpenOptions {
        read: true,
        write: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    ops.open_file(path.parse().unwrap(), options).await.unwrap()
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(
    #[values(
    // "file_does_not_exist",
    "file_exists_offset_too_high",
    // "file_exists_overwrite",
    // "file_exists_truncate",
)]
    kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // 1) Open the file and do the write

    let (options, path, offset, expected_content) = {
        let mut options = OpenOptions {
            read: false,
            write: true,
            truncate: false,
            create: false,
            create_new: false,
        };
        match kind {
            "file_does_not_exist" => {
                options.create = true;
                (options, "/new.txt", 0, b"new".as_ref())
            }
            "file_exists_offset_too_high" => {
                (options, "/bar.txt", 13, b"hello world\x00\x00new".as_ref())
            }
            // cspell:disable-next-line
            "file_exists_overwrite" => (options, "/bar.txt", 0, b"newlo world".as_ref()),
            "file_exists_truncate" => {
                options.truncate = true;
                (options, "/bar.txt", 0, b"new".as_ref())
            }
            unknown => panic!("Unknown kind: {}", unknown),
        }
    };
    let fd1 = ops.open_file(path.parse().unwrap(), options).await.unwrap();
    p_assert_eq!(fd1.0, 1);

    let spy = ops.event_bus.spy.start_expecting();

    let written = ops.fd_write(fd1, offset, b"new").await.unwrap();
    p_assert_eq!(written, 3);

    spy.assert_no_events(); // Event is only triggered on file close

    // 2) Check the file contains the expected content

    let fd2 = open_for_read(&ops, path).await;
    p_assert_eq!(fd2.0, 2);
    let mut buf = Vec::with_capacity(expected_content.len());
    let read = ops
        .fd_read(fd2, 0, expected_content.len() as u64, &mut buf)
        .await
        .unwrap();
    p_assert_eq!(buf, expected_content);
    p_assert_eq!(read, expected_content.len() as u64);

    // 3) Ensure the write is not tied to file descriptor not being closed

    ops.fd_close(fd1).await.unwrap();
    ops.fd_close(fd2).await.unwrap();

    let fd3 = open_for_read(&ops, path).await;
    p_assert_eq!(fd3.0, 3);
    let mut buf = Vec::with_capacity(expected_content.len());
    let read = ops
        .fd_read(fd3, 0, expected_content.len() as u64, &mut buf)
        .await
        .unwrap();
    p_assert_eq!(buf, expected_content);
    p_assert_eq!(read, expected_content.len() as u64);

    ops.fd_close(fd3).await.unwrap();

    // 4) Restart the workspace to ensure the write is persistent

    let ops = restart_workspace_ops(ops).await;

    let fd4 = open_for_read(&ops, path).await;
    p_assert_eq!(fd4.0, 1); // New ops, so file descriptor counter is reset
    let mut buf = Vec::with_capacity(expected_content.len());
    let read = ops
        .fd_read(fd4, 0, expected_content.len() as u64, &mut buf)
        .await
        .unwrap();
    p_assert_eq!(buf, expected_content);
    p_assert_eq!(read, expected_content.len() as u64);

    ops.fd_close(fd4).await.unwrap();
    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn cursor_not_in_write_mode(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let path = "/bar.txt".parse().unwrap();
    let options = OpenOptions {
        read: false,
        write: false,
        truncate: false,
        create: false,
        create_new: false,
    };
    let fd = ops.open_file(path, options).await.unwrap();
    p_assert_eq!(fd.0, 1);

    let err = ops.fd_write(fd, 0, b"foo").await.unwrap_err();
    spy.assert_no_events();
    p_assert_matches!(err, WorkspaceFdWriteError::NotInWriteMode);

    ops.fd_close(fd).await.unwrap();
    ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn unknown_file_descriptor(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let spy = ops.event_bus.spy.start_expecting();

    let err = ops
        .fd_write(FileDescriptor(1), 0, b"foo")
        .await
        .unwrap_err();
    spy.assert_no_events();
    p_assert_matches!(err, WorkspaceFdWriteError::BadFileDescriptor);

    ops.stop().await.unwrap();
}
