// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{workspace_history::WorkspaceHistoryOps, Client};
use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::mount_history_and_test;

#[parsec_test(testbed = "workspace_history")]
async fn file(
    #[values("file", "folder", "not_found")] kind: &'static str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_v2_timestamp: DateTime = *env.template.get_stuff("wksp1_v2_timestamp");

    mount_history_and_test!(
        at: wksp1_v2_timestamp,
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            match kind {
                "file" => {
                    test_register_sequence_of_send_hooks!(
                        &env.discriminant_dir,
                        // Workspace manifest is always guaranteed to be in cache
                        // Workspace key bundle has already been loaded at workspace history ops startup
                        // 1) Resolve `/bar.txt` path: get back the `bar.txt` manifest
                        test_send_hook_vlob_read_batch!(env, at: wksp1_v2_timestamp, wksp1_id, wksp1_bar_txt_id),
                    );

                    let stat = tokio::fs::metadata(mountpoint_path.join("bar.txt")).await.unwrap();
                    assert!(stat.is_file());
                    p_assert_eq!(stat.len(), 8);
                }

                "folder" => {
                    test_register_sequence_of_send_hooks!(
                        &env.discriminant_dir,
                        // Workspace manifest is always guaranteed to be in cache
                        // Workspace key bundle has already been loaded at workspace history ops startup
                        // 1) Resolve `/foo` path: get back the `foo` manifest
                        test_send_hook_vlob_read_batch!(env, at: wksp1_v2_timestamp, wksp1_id, wksp1_foo_id),
                    );

                    let stat = tokio::fs::metadata(mountpoint_path.join("foo")).await.unwrap();
                    assert!(stat.is_dir());
                }

                "not_found" => {
                    let err = tokio::fs::metadata(mountpoint_path.join("dummy")).await.unwrap_err();
                    p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
                }

                unknown => panic!("Unknown kind: {}", unknown),
            };
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
            let err = tokio::fs::metadata(mountpoint_path.join("bar.txt")).await.unwrap_err();

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
