// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{os::linux::fs::MetadataExt, path::PathBuf, sync::Arc};

use libparsec_client::{Client, WorkspaceOps};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn unmount_with_fusermount(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // `st_dev` is the device number of the filesystem, hence it will change after unmounting
            let mounted_st_dev = tokio::fs::metadata(&mountpoint_path)
                .await
                .unwrap()
                .st_dev();

            // `-z` option is important here: it's the "lazy umount" mode, which allows the
            // filesystem to be unmounted even if it is still in use.
            // This is important given upon mounting, the filesystem gets polled by various
            // system that can clash with our attempt at unmounting.
            assert!(std::process::Command::new("fusermount")
                .args(["-uz", mountpoint_path.to_str().unwrap()])
                .status()
                .unwrap()
                .success());

            for _ in 0..10 {
                let unmounted_st_dev = tokio::fs::metadata(&mountpoint_path)
                    .await
                    .unwrap()
                    .st_dev();
                if unmounted_st_dev != mounted_st_dev {
                    // All done !
                    return;
                }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
            panic!("Unmount seems to have failed, st_dev is still the same");
        }
    );
}
