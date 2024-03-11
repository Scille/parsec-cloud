// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{
    workspace::{EntryStat, WorkspaceOps},
    Client,
};
use libparsec_tests_fixtures::prelude::*;
use tokio::io::AsyncWriteExt;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(
    #[values("read_only", "nothing_to_flush", "something_to_flush")] kind: &str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut open_options = tokio::fs::OpenOptions::new();
            let (something_to_flush, expected_size) = match kind {
                "read_only" => {
                    open_options.read(true);
                    (false, "hello world".len() as u64)
                }
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

            let mut fd = open_options
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            if something_to_flush {
                fd.write_all(b"123").await.unwrap();
                fd.write_all(b"123").await.unwrap();
            }

            fd.flush().await.unwrap();

            let stat = wksp1_ops
                .stat_entry(&"/bar.txt".parse().unwrap())
                .await
                .unwrap();
            p_assert_matches!(stat, EntryStat::File { size, .. } if size == expected_size);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .write(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();
            fd.write_all(b"123456").await.unwrap();

            client.stop_workspace(wksp1_ops.realm_id()).await;
            let err = fd.flush().await.unwrap_err();
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
