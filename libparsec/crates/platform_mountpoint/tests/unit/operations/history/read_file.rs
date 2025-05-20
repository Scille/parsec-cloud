// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    io::{Read, Seek},
    path::PathBuf,
    sync::Arc,
};

use libparsec_client::{Client, workspace_history::WorkspaceHistoryOps};
use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_block_read, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::mount_history_and_test;

// TODO: test block access referencing a non-existing block
// TODO: test block data and block access hash mismatch

#[parsec_test(testbed = "workspace_history")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_v1_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v1_block_access");
    let wksp1_bar_txt_v2_block1_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v2_block1_access");
    let wksp1_bar_txt_v2_block2_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v2_block2_access");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");
    let wksp1_bar_txt_v2_timestamp: DateTime =
        *env.template.get_stuff("wksp1_bar_txt_v2_timestamp");

    mount_history_and_test!(
        at: wksp1_v2_timestamp,
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // Workspace manifest is always guaranteed to be in cache
                // Workspace key bundle has already been loaded at workspace history ops startup
                // 1) Resolve `/bar.txt` path: get back the `bar.txt` manifest
                test_send_hook_vlob_read_batch!(env, at: wksp1_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
                // 2) Get the block data referenced in `bar.txt` v1 manifest
                test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v1_block_access.id),
            );

            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let bar_path = mountpoint_path.join("bar.txt");
            tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(bar_path)
                    .unwrap();

                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap();
                p_assert_eq!(buff, b"Hello v1");

                // Read with cursor exhausted
                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap();
                p_assert_eq!(buff, b"");

                // Re-wind the cursor, and read a part of the file
                fd.seek(std::io::SeekFrom::Start(3)).unwrap();
                let mut buff = vec![0; 4];
                fd.read_exact(&mut buff).unwrap();
                p_assert_eq!(buff, b"lo v");
            })
            .await
            .unwrap();

            // Change timestamp of interest and read the file again

            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // Get back `/` manifest
                test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v2_timestamp, wksp1_id, wksp1_id),
                // Note workspace key bundle has already been loaded at workspace history ops startup
            );
            wksp1_ops.set_timestamp_of_interest(wksp1_bar_txt_v2_timestamp).await.unwrap();

            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // Workspace manifest is always guaranteed to be in cache
                // Workspace key bundle has already been loaded at workspace history ops startup
                // 1) Resolve `/bar.txt` path: get back the `bar.txt` manifest
                test_send_hook_vlob_read_batch!(env, at: wksp1_bar_txt_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
                // 2) Get the blocks data referenced in `bar.txt` v2 manifest
                test_send_hook_block_read!(env, wksp1_id, allowed: [
                    wksp1_bar_txt_v2_block1_access.id,
                    wksp1_bar_txt_v2_block2_access.id,
                ]),
                test_send_hook_block_read!(env, wksp1_id, allowed: [
                    wksp1_bar_txt_v2_block1_access.id,
                    wksp1_bar_txt_v2_block2_access.id,
                ]),
            );

            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let bar_path = mountpoint_path.join("bar.txt");
            tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(bar_path)
                    .unwrap();

                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap();
                p_assert_eq!(buff, b"Hello v2 world");

                // Read with cursor exhausted
                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap();
                p_assert_eq!(buff, b"");

                // Re-wind the cursor, and read a part of the file
                fd.seek(std::io::SeekFrom::Start(3)).unwrap();
                let mut buff = vec![0; 6];
                fd.read_exact(&mut buff).unwrap();
                p_assert_eq!(buff, b"lo v2 ");
            })
            .await
            .unwrap();
        }
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn read_too_much(tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_bar_txt_v1_block_access: &BlockAccess =
        env.template.get_stuff("wksp1_bar_txt_v1_block_access");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");

    mount_history_and_test!(
        at: wksp1_v2_timestamp,
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // Workspace manifest is always guaranteed to be in cache
                // Workspace key bundle has already been loaded at workspace history ops startup
                // 1) Resolve `/bar.txt` path: get back the `bar.txt` manifest
                test_send_hook_vlob_read_batch!(env, at: wksp1_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
                // 2) Get the block data referenced in `bar.txt` v1 manifest
                test_send_hook_block_read!(env, wksp1_id, wksp1_bar_txt_v1_block_access.id),
            );

            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let err = tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();

                let mut buff = vec![0; 20];
                fd.read_exact(&mut buff).unwrap_err()
            })
            .await
            .unwrap();

            p_assert_matches!(err.kind(), std::io::ErrorKind::UnexpectedEof);
        }
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn offline(tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");

    mount_history_and_test!(
        at: wksp1_v2_timestamp,
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // Workspace manifest is always guaranteed to be in cache
                // Workspace key bundle has already been loaded at workspace history ops startup
                // 1) Resolve `/bar.txt` path: get back the `bar.txt` manifest
                test_send_hook_vlob_read_batch!(env, at: wksp1_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
                // 2) No mock for getting the block data referenced in `bar.txt` v1 manifest, so we end up offline !
            );

            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let bar_path = mountpoint_path.join("bar.txt");
            tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(bar_path)
                    .unwrap();

                let mut buff = Vec::new();
                let err = fd.read_to_end(&mut buff).unwrap_err();

                // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
                #[cfg(not(target_os = "windows"))]
                p_assert_eq!(err.raw_os_error(), Some(libc::EHOSTUNREACH), "{}", err);
                #[cfg(target_os = "windows")]
                p_assert_eq!(
                    err.raw_os_error(),
                    Some(windows_sys::Win32::Foundation::ERROR_HOST_UNREACHABLE as i32),
                    "{}",
                    err
                );
            })
            .await
            .unwrap();
        }
    );
}
