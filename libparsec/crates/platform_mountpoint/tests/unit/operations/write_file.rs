// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

use libparsec_client::{workspace::WorkspaceOps, Client};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{mount_and_test, ops_cat};

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_overwrite(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .write(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            fd.write_all(b"01234").await.unwrap();
            fd.seek(tokio::io::SeekFrom::Start(7)).await.unwrap();
            fd.write_all(b"0").await.unwrap();
            // Flush guarantees that the write is pushed up to the filesystem
            fd.flush().await.unwrap();

            p_assert_eq!(
                ops_cat!(wksp1_ops.clone(), "/bar.txt").await,
                b"01234 w0rld"
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_append(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            fd.write_all(b"01234").await.unwrap();
            fd.write_all(b"56789").await.unwrap();
            // Flush guarantees that the write is pushed up to the filesystem
            fd.flush().await.unwrap();

            p_assert_eq!(
                ops_cat!(wksp1_ops.clone(), "/bar.txt").await,
                b"hello world0123456789"
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_truncate(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            fd.write_all(b"01234").await.unwrap();
            // Flush guarantees that the write is pushed up to the filesystem
            fd.flush().await.unwrap();

            p_assert_eq!(ops_cat!(wksp1_ops.clone(), "/bar.txt").await, b"01234");
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_write_past_end(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .write(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            fd.seek(std::io::SeekFrom::End(10)).await.unwrap();
            fd.write_all(b"0").await.unwrap();
            // Flush guarantees that the write is pushed up to the filesystem
            fd.flush().await.unwrap();

            p_assert_eq!(
                ops_cat!(wksp1_ops.clone(), "/bar.txt").await,
                b"hello world\x00\x00\x00\x00\x00\x00\x00\x00\x00\x000"
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_in_write_mode(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            let maybe_error = fd.write_all(b"01234").await;
            // Write is not guaranteed to be pushed up to the filesystem (it can be kept
            // in some buffer on application side), in that case we need to do a flush.
            let err = match maybe_error {
                Ok(_) => {
                    let maybe_error = fd.write_all(b"01234").await;
                    match maybe_error {
                        Ok(_) => fd.flush().await.unwrap_err(),
                        Err(err) => err,
                    }
                }
                Err(err) => err,
            };

            p_assert_eq!(err.raw_os_error(), Some(libc::EBADF), "{}", err);
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

            client.stop_workspace(wksp1_ops.realm_id()).await;

            // Write operation only store data in memory, so stopping the workspace
            // doesn't affect it !
            fd.write_all(b"01234").await.unwrap();
        }
    );
}
