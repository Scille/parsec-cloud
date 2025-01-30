// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client::{Client, WorkspaceOps};
use libparsec_tests_fixtures::prelude::*;
use std::{path::PathBuf, sync::Arc};

use crate::clean_base_mountpoint_dir;

use super::utils::{mount_and_test, os_ls};

#[parsec_test(testbed = "coolorg")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    let mountpoint_base_dir: &libparsec_tests_fixtures::TmpPath = &tmp_path;
    let mountpoint_base_dir: std::path::PathBuf = (*mountpoint_base_dir).to_owned();
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, _mountpoint_path: PathBuf| async move {
            // 1) Create some directories inside base mountpoint directory
            // TODO: create an artifact by starting a Parsec client and kill it? How?

            std::fs::create_dir_all(mountpoint_base_dir.join("non_empty_dir/some_dir")).unwrap();
            std::fs::create_dir_all(mountpoint_base_dir.join("empty_dir")).unwrap();

            // 2) Check initial status of base mountpoint directory

            let mut entries = os_ls!(&mountpoint_base_dir).await;
            entries.sort();
            p_assert_eq!(
                entries,
                [
                    "empty_dir",     // Regular empty directory
                    "non_empty_dir", // Regular non-empty directory
                    "wksp1"          // Empty directory with a valid mountpoint
                ]
            );

            // 3) Perform cleanup routine

            clean_base_mountpoint_dir(mountpoint_base_dir.clone())
                .await
                .unwrap();

            // 4) Only empty dirs and artifacts are removed

            entries = os_ls!(&mountpoint_base_dir).await;
            entries.sort();
            p_assert_eq!(entries, ["non_empty_dir", "wksp1"]);
        }
    );
}
