// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{
    workspace::{WorkspaceOps, WorkspaceStatEntryError},
    Client,
};
use libparsec_tests_fixtures::prelude::*;

use super::utils::mount_and_test;

// TODO: move is not supported yet :'(
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_file(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let src = mountpoint_path.join("bar.txt");
            let dst = mountpoint_path.join("foo/bar2.txt");

            tokio::fs::rename(&src, &dst).await.unwrap();

            let err = tokio::fs::metadata(&src).await.unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);

            p_assert_eq!(tokio::fs::read(&dst).await.unwrap(), b"hello world");

            p_assert_matches!(
                wksp1_ops
                    .stat_entry(&"/bar.txt".parse().unwrap())
                    .await
                    .unwrap_err(),
                WorkspaceStatEntryError::EntryNotFound
            );
            wksp1_ops
                .stat_entry(&"/foo/bar2.txt".parse().unwrap())
                .await
                .unwrap();
        }
    );
}

// TODO: move is not supported yet :'(
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_folder(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            wksp1_ops
                .create_folder("/dst".parse().unwrap())
                .await
                .unwrap();

            let src = mountpoint_path.join("foo");
            let dst = mountpoint_path.join("dst/foo2");

            tokio::fs::rename(&src, &dst).await.unwrap();

            p_assert_matches!(
                wksp1_ops
                    .stat_entry(&"/foo".parse().unwrap())
                    .await
                    .unwrap_err(),
                WorkspaceStatEntryError::EntryNotFound
            );
            wksp1_ops
                .stat_entry(&"/dst/foo2".parse().unwrap())
                .await
                .unwrap();

            // Ensure mountpoint has a correct view on data

            let err = tokio::fs::metadata(&src).await.unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);

            // Ensure child has been moved as well

            for child_name in ["egg.txt", "spam"] {
                let err = tokio::fs::metadata(&src.join(child_name))
                    .await
                    .unwrap_err();
                p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);

                tokio::fs::metadata(&dst.join(child_name)).await.unwrap();
            }
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn src_not_found(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let src = mountpoint_path.join("dummy");
            let dst = mountpoint_path.join("foo2");

            let err = tokio::fs::rename(&src, &dst).await.unwrap_err();

            p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn dst_parent_doesnt_exist(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let src = mountpoint_path.join("foo");
            let dst = mountpoint_path.join("dummy/foo2");

            let err = tokio::fs::rename(&src, &dst).await.unwrap_err();

            p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
        }
    );
}

// TODO: `tokio::fs::rename` always overwrite target, however on Windows there
//       is a `MOVEFILE_REPLACE_EXISTING` flag to control this behavior
//       (see https://learn.microsoft.com/fr-fr/windows/win32/api/winbase/nf-winbase-movefileexw)
#[cfg(not(target_os = "windows"))]
#[parsec_test(testbed = "minimal_client_ready")]
async fn dst_parent_exists_as_file(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let src = mountpoint_path.join("foo");
            let dst = mountpoint_path.join("bar.txt");

            let err = tokio::fs::rename(&src, &dst).await.unwrap_err();

            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::ENOTDIR), "{}", err);
            #[cfg(target_os = "windows")]
            {
                p_assert_eq!(
                    err.raw_os_error(),
                    Some(windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS as i32),
                    "{}",
                    err
                );
            }
        }
    );
}

// TODO: `tokio::fs::rename` always overwrite target, however on Windows there
//       is a `MOVEFILE_REPLACE_EXISTING` flag to control this behavior
//       (see https://learn.microsoft.com/fr-fr/windows/win32/api/winbase/nf-winbase-movefileexw)
#[cfg(not(target_os = "windows"))]
#[parsec_test(testbed = "minimal_client_ready")]
async fn dst_parent_exists_as_folder(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let src = mountpoint_path.join("bar.txt");
            let dst = mountpoint_path.join("foo");

            let err = tokio::fs::rename(&src, &dst).await.unwrap_err();

            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EISDIR), "{}", err);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS as i32),
                "{}",
                err
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn dst_parent_is_file(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let src = mountpoint_path.join("foo");
            let dst = mountpoint_path.join("bar.txt/foo2");

            let err = tokio::fs::rename(&src, &dst).await.unwrap_err();

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

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            client.stop_workspace(wksp1_ops.realm_id()).await;

            let src = mountpoint_path.join("bar.txt");
            let dst = mountpoint_path.join("bar2.txt");

            let err = tokio::fs::rename(&src, &dst).await.unwrap_err();
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EIO), "{}", err);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_NOT_READY as i32),
                "{}",
                err
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(tmp_path: TmpPath, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        // Ignore all events related to workspace local storage except for the
        // workspace manifest. This way we have a root containing entries, but
        // accessing them require to fetch data from the server.
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
                    | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
                    | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageLocalWorkspaceManifestUpdate(_)
                    | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
                    | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_)
                    | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
    });
    mount_and_test!(
        &env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let src = mountpoint_path.join("bar.txt");
            let dst = mountpoint_path.join("bar2.txt");

            let err = tokio::fs::rename(&src, &dst).await.unwrap_err();
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
