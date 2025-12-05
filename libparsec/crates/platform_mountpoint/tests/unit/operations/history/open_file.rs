// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{workspace_history::WorkspaceHistoryOps, Client};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::mount_history_and_test;

// TODO: test invalid file manifest

#[parsec_test(testbed = "workspace_history")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
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
            );

            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            tokio::task::spawn_blocking(move || {
                std::fs::OpenOptions::new().read(true).open(mountpoint_path.join("bar.txt")).unwrap();
            })
            .await
            .unwrap();
        }
    );
}

#[parsec_test(testbed = "workspace_history")]
#[case("truncate_existing")]
#[cfg_attr(
    target_os = "windows",
    ignore = "Ignoring because often the test is stuck on the CI CF issue #11842"
)]
#[case("create_on_existing")]
#[case("create_on_non_existing")]
#[case("create_new_on_non_existing")]
async fn read_only(#[case] kind: &str, tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");

    mount_history_and_test!(
        at: wksp1_v2_timestamp,
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            let mut open_options = std::fs::OpenOptions::new();
            let (name, bar_is_fetched) = match kind {
                "truncate_existing" => {
                    open_options.write(true);
                    open_options.truncate(true);
                    ("bar.txt", true)
                }
                "create_on_existing" => {
                    open_options.write(true);
                    open_options.create(true);
                    ("bar.txt", true)
                }
                "create_on_non_existing" => {
                    open_options.write(true);
                    open_options.create(true);
                    ("new.txt", false)
                }
                "create_new_on_non_existing" => {
                    open_options.write(true);
                    open_options.create_new(true);
                    ("new.txt", false)
                }
                unknown => panic!("Unknown kind: {unknown}"),
            };

            if bar_is_fetched {
                test_register_sequence_of_send_hooks!(
                    &env.discriminant_dir,
                    // Workspace manifest is always guaranteed to be in cache
                    // Workspace key bundle has already been loaded at workspace history ops startup
                    // 1) Resolve `/bar.txt` path: get back the `bar.txt` manifest
                    test_send_hook_vlob_read_batch!(env, at: wksp1_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
                );
            }

            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let err = tokio::task::spawn_blocking(move || {
                open_options.open(mountpoint_path.join(name)).unwrap_err()
            })
            .await.unwrap();

            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            // TODO: what should be the error here?
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EROFS), "{}", err);
            // In theory we would expect `ERROR_WRITE_PROTECT` here, however it causes
            // a "Catastrophic Failure" error in Windows Explorer, so instead we use
            // `ERROR_ACCESS_DENIED`.
            #[cfg(target_os = "windows")]
            p_assert_eq!(err.raw_os_error(), Some(windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED as i32), "{}", err);
        }
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn not_found(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_history_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            let err = tokio::fs::read(&mountpoint_path.join("dummy.txt"))
                .await
                .unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
        }
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn offline(tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");

    mount_history_and_test!(
        at: wksp1_v2_timestamp,
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            let err = tokio::fs::OpenOptions::new().read(true)
                .open(mountpoint_path.join("bar.txt"))
                .await
                .unwrap_err();

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
        }
    );
}
