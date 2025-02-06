// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{workspace_history::WorkspaceHistoryOps, Client};
use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vlob_read_batch,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::{mount_history_and_test, os_ls};

#[parsec_test(testbed = "workspace_history")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_v2_children_available_timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Resolve `/` path: get back the workspace manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_id),
        // Note workspace key bundle has already been loaded at workspace history ops startup
        // 2) Resolve `/foo` path: get back the `foo` manifest
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, wksp1_foo_id),
        // 3) Get `/foo/egg.txt` & `/foo/spam` manifest (order of fetch is not guaranteed)
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
        test_send_hook_vlob_read_batch!(env, at: wksp1_foo_v2_children_available_timestamp, wksp1_id, allowed: [wksp1_foo_egg_txt_id, wksp1_foo_spam_id]),
    );

    mount_history_and_test!(
        at wksp1_foo_v2_children_available_timestamp,
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceHistoryOps>, mountpoint_path: PathBuf| async move {
            // let mut items = os_ls!(mountpoint_path).await;
            // // Children are stored in the workspace/folder manifests as a hashmap, so
            // // the order of iteration is not stable between runs...
            // items.sort();
            // p_assert_eq!(items, ["bar.txt", "foo"]);
        }
    );
}

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn ignore_invalid_children(tmp_path: TmpPath, env: &TestbedEnv) {
//     let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
//     let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
//     let bad_parent_id = wksp1_foo_egg_txt_id;
//     env.customize(|builder| {
//         builder
//             .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
//             .customize_children(
//                 [
//                     // Existing entry, but with a parent field not pointing to us
//                     ("bad_parent.txt", Some(bad_parent_id)),
//                 ]
//                 .into_iter(),
//             );
//     })
//     .await;

//     mount_and_test!(
//         env,
//         &tmp_path,
//         |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
//             let mut items = os_ls!(mountpoint_path).await;
//             // Children are stored in the workspace/folder manifests as a hashmap, so
//             // the order of iteration is not stable between runs...
//             items.sort();
//             p_assert_eq!(items, ["bar.txt", "foo"]);
//         }
//     );
// }

// // FUSE typically uses readdir to read a block of 128 entries at a time.
// #[parsec_test(testbed = "minimal_client_ready")]
// async fn ok_lot_of_entries(tmp_path: TmpPath, env: &TestbedEnv) {
//     mount_and_test!(
//         env,
//         &tmp_path,
//         |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
//             for i in 0..1000 {
//                 let path = format!("/foo/spam/{}", i).parse().unwrap();
//                 wksp1_ops.create_file(path).await.unwrap();
//             }

//             let mut children: Vec<_> = os_ls!(mountpoint_path.join("foo/spam"))
//                 .await
//                 .into_iter()
//                 .map(|n| n.parse::<u64>().unwrap())
//                 .collect();
//             p_assert_eq!(
//                 children.len(),
//                 1000,
//                 "Expected 1000 entries, got {}",
//                 children.len()
//             );
//             children.sort();
//             p_assert_eq!(children, (0..1000).collect::<Vec<_>>());
//         }
//     );
// }

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn not_found(tmp_path: TmpPath, env: &TestbedEnv) {
//     mount_and_test!(
//         env,
//         &tmp_path,
//         |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
//             let err = tokio::fs::read_dir(mountpoint_path.join("dummy"))
//                 .await
//                 .unwrap_err();
//             p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
//         }
//     );
// }

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn is_file(tmp_path: TmpPath, env: &TestbedEnv) {
//     mount_and_test!(
//         env,
//         &tmp_path,
//         |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
//             let err = tokio::fs::read_dir(mountpoint_path.join("bar.txt"))
//                 .await
//                 .unwrap_err();
//             #[cfg(not(target_os = "windows"))]
//             p_assert_eq!(err.raw_os_error(), Some(libc::ENOTDIR), "{}", err);
//             #[cfg(target_os = "windows")]
//             p_assert_eq!(
//                 err.raw_os_error(),
//                 Some(windows_sys::Win32::Foundation::ERROR_DIRECTORY as i32),
//                 "{}",
//                 err
//             );
//         }
//     );
// }

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn stopped(tmp_path: TmpPath, env: &TestbedEnv) {
//     mount_and_test!(
//         env,
//         &tmp_path,
//         |client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
//             client.stop_workspace(wksp1_ops.realm_id()).await;

//             let err = tokio::fs::read_dir(mountpoint_path.join("foo"))
//                 .await
//                 .unwrap_err();
//             #[cfg(not(target_os = "windows"))]
//             p_assert_eq!(err.raw_os_error(), Some(libc::EIO), "{}", err);
//             #[cfg(target_os = "windows")]
//             p_assert_eq!(
//                 err.raw_os_error(),
//                 Some(windows_sys::Win32::Foundation::ERROR_NOT_READY as i32),
//                 "{}",
//                 err
//             );
//         }
//     );
// }

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn offline(tmp_path: TmpPath, env: &TestbedEnv) {
//     env.customize(|builder| {
//         // Ignore all events related to workspace local storage except for the
//         // workspace manifest. This way we have a root containing entries, but
//         // accessing them require to fetch data from the server.
//         builder.filter_client_storage_events(|e| match e {
//             TestbedEvent::WorkspaceDataStorageFetchFolderVlob(e)
//                 if e.local_manifest.base.is_root() =>
//             {
//                 true
//             }
//             TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
//             | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
//             | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
//             | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
//             | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_)
//             | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_)
//             | TestbedEvent::WorkspaceDataStorageChunkCreate(_) => false,
//             _ => true,
//         });
//     })
//     .await;
//     mount_and_test!(
//         env,
//         &tmp_path,
//         |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
//             let err = tokio::fs::read_dir(mountpoint_path.join("foo"))
//                 .await
//                 .unwrap_err();
//             // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
//             #[cfg(not(target_os = "windows"))]
//             p_assert_eq!(err.raw_os_error(), Some(libc::EHOSTUNREACH), "{}", err);
//             #[cfg(target_os = "windows")]
//             p_assert_eq!(
//                 err.raw_os_error(),
//                 Some(windows_sys::Win32::Foundation::ERROR_HOST_UNREACHABLE as i32),
//                 "{}",
//                 err
//             );
//         }
//     );
// }
