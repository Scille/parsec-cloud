// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{
    workspace::{WorkspaceOps, WorkspaceStatEntryError},
    Client,
};
use libparsec_tests_fixtures::prelude::*;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_file(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let src = mountpoint_path.join("bar.txt");
            let dst = mountpoint_path.join("bar2.txt");

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
                .stat_entry(&"/bar2.txt".parse().unwrap())
                .await
                .unwrap();
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_folder(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let src = mountpoint_path.join("foo");
            let dst = mountpoint_path.join("foo2");

            tokio::fs::rename(&src, &dst).await.unwrap();

            p_assert_matches!(
                wksp1_ops
                    .stat_entry(&"/foo".parse().unwrap())
                    .await
                    .unwrap_err(),
                WorkspaceStatEntryError::EntryNotFound
            );
            wksp1_ops
                .stat_entry(&"/foo2".parse().unwrap())
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
