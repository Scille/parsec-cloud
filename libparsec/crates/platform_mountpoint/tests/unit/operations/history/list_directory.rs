// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{workspace_history::WorkspaceHistoryOps, Client};
use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::{mount_history_and_test, os_ls};

// TODO: test invalid child manifest
// TODO: test child manifest with non-matching parent ID

#[parsec_test(testbed = "workspace_history")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    mount_history_and_test!(
        at: wksp1_foo_v2_children_available_timestamp,
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {

            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // Workspace manifest is always guaranteed to be in cache
                // Workspace key bundle has already been loaded at workspace history ops startup
                // 1) Resolve `/foo` path: get back the `foo` manifest
                test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_foo_id),
                // 2) Get `/foo/egg.txt` & `/foo/spam` manifest (order of fetch is not guaranteed)
                test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
                test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
            );

            let mut items = os_ls!(mountpoint_path.join("foo")).await;
            // Children are stored in the workspace/folder manifests as a hashmap, so
            // the order of iteration is not stable between runs...
            items.sort();
            p_assert_eq!(items, ["egg.txt", "spam"]);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_found(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_history_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            let err = tokio::fs::read_dir(mountpoint_path.join("dummy"))
                .await
                .unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
        }
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn is_file(tmp_path: TmpPath, env: &TestbedEnv) {
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

            let err = tokio::fs::read_dir(mountpoint_path.join("bar.txt"))
                .await
                .unwrap_err();

            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::ENOTDIR), "{}", err);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_DIRECTORY as i32),
                "{}",
                err
            );
        }
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn offline(tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    mount_history_and_test!(
        at: wksp1_foo_v2_children_available_timestamp,
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            let err = match tokio::fs::read_dir(mountpoint_path.clone()).await {
                Ok(mut reader) => reader.next_entry().await.unwrap_err(),
                Err(err) => err,
            };

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
