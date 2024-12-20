// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{io::Write, path::PathBuf, sync::Arc};

use libparsec_client::{
    workspace::{EntryStat, WorkspaceOps},
    Client,
};
use libparsec_tests_fixtures::prelude::*;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(
    #[values("nothing_to_flush", "something_to_flush")] kind: &str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut open_options = std::fs::OpenOptions::new();
            let (something_to_flush, expected_size) = match kind {
                "nothing_to_flush" => {
                    open_options.write(true);
                    (false, "hello world".len() as u64)
                }
                "something_to_flush" => {
                    open_options.write(true);
                    (true, "123123world".len() as u64)
                }
                unknown => panic!("Unknown kind: {}", unknown),
            };

            // Using `tokio::fs::File::sync_data` cause a deadlock if the tokio runtime
            // is single threaded, so we have to use threads manually instead.
            let fd = tokio::task::spawn_blocking(move || {
                let mut fd = open_options.open(mountpoint_path.join("bar.txt")).unwrap();

                if something_to_flush {
                    fd.write_all(b"123").unwrap();
                    fd.write_all(b"123").unwrap();
                }

                fd.sync_data().unwrap();

                fd
            })
            .await
            .unwrap();

            let stat = wksp1_ops
                .stat_entry(&"/bar.txt".parse().unwrap())
                .await
                .unwrap();
            p_assert_matches!(stat, EntryStat::File { base, .. } if base.size == expected_size);

            // File descriptor must be kept so that the file is not closed before we do the stat
            tokio::task::spawn_blocking(move || {
                drop(fd);
            })
            .await
            .unwrap();
        }
    );
}
#[parsec_test(testbed = "minimal_client_ready")]
async fn read_only(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Using `tokio::fs::File::sync_data` cause a deadlock if the tokio runtime
            // is single threaded, so we have to use threads manually instead.
            let err = tokio::task::spawn_blocking(move || {
                let fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();

                fd.sync_data()
            })
            .await
            .unwrap()
            .unwrap_err();

            p_assert_matches!(err.kind(), std::io::ErrorKind::PermissionDenied);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Using `tokio::fs::File::sync_data` cause a deadlock if the tokio runtime
            // is single threaded, so we have to use threads manually instead.
            let fd = tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .write(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();
                fd.write_all(b"123456").unwrap();
                fd
            })
            .await
            .unwrap();

            client.stop_workspace(wksp1_ops.realm_id()).await;

            let err = tokio::task::spawn_blocking(move || fd.sync_data())
                .await
                .unwrap()
                .unwrap_err();

            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EIO), "{}", err);
            // TODO: error is expected to be `ERROR_NOT_READY`, but due to lookup
            //       before actually doing the flush, the error is different
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_INVALID_HANDLE as i32),
                "{}",
                err
            );
        }
    );
}
