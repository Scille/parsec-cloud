// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: setattr is not supported, but must test the behavior nevertheless
// - Resize file (increase/decrease) with/out file descriptor
// - Change access/modification time with/out file descriptor

use std::path::PathBuf;

use libparsec_tests_fixtures::{tmp_path, TestbedEnv, TmpPath};
use libparsec_tests_lite::parsec_test;

use crate::operations::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn unhandled_attr_change_nothing(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client, _wksp_ops, mountpoint_path: PathBuf| async move {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};

            let file_path = mountpoint_path.join("bar.txt");
            let orig_metadata = tokio::fs::metadata(&file_path).await.unwrap();
            tokio::fs::set_permissions(&file_path, std::fs::Permissions::from_mode(0o755))
                .await
                .unwrap();
            let new_metadata = tokio::fs::metadata(&file_path).await.unwrap();
            assert_eq!(orig_metadata.file_type(), new_metadata.file_type());
            assert_eq!(
                orig_metadata.accessed().unwrap(),
                new_metadata.accessed().unwrap()
            );
            // FIXME: Currently the filesystem does not support creation time
            // The following code will panic with the above reason
            // assert_eq!(
            //     orig_metadata.created().unwrap(),
            //     new_metadata.created().unwrap()
            // );
            assert_eq!(
                orig_metadata.modified().unwrap(),
                new_metadata.modified().unwrap()
            );
            assert_eq!(orig_metadata.len(), new_metadata.len());
            assert_eq!(orig_metadata.permissions(), new_metadata.permissions());
            assert_eq!(orig_metadata.dev(), new_metadata.dev());
            assert_eq!(orig_metadata.ino(), new_metadata.ino());
            assert_eq!(orig_metadata.mode(), new_metadata.mode());
            assert_eq!(orig_metadata.nlink(), new_metadata.nlink());
            assert_eq!(orig_metadata.uid(), new_metadata.uid());
            assert_eq!(orig_metadata.gid(), new_metadata.gid());
            assert_eq!(orig_metadata.rdev(), new_metadata.rdev());
            assert_eq!(orig_metadata.atime(), new_metadata.atime());
            assert_eq!(orig_metadata.atime_nsec(), new_metadata.atime_nsec());
            assert_eq!(orig_metadata.mtime(), new_metadata.mtime());
            assert_eq!(orig_metadata.mtime_nsec(), new_metadata.mtime_nsec());
            assert_eq!(orig_metadata.ctime(), new_metadata.ctime());
            assert_eq!(orig_metadata.ctime_nsec(), new_metadata.ctime_nsec());
            assert_eq!(orig_metadata.blksize(), new_metadata.blksize());
        }
    );
}
