// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::WorkspaceOps;
use libparsec_tests_fixtures::{tmp_path, TestbedEnv, TmpPath};
use libparsec_tests_lite::parsec_test;

use crate::operations::utils::mount_and_test;

// NOTE: We cannot test the behavoir of readlink since our filesystem does not support links.
// Calling `readlink` with a file will just result in the error
// [`std::io::ErrorKind::InvalidInput`] because the "system" will actual do a pre-check with
// `lookup` on the path and discover that it is not a link.

#[parsec_test(testbed = "minimal_client_ready")]
async fn linkat_not_supported(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let result = tokio::fs::hard_link(
                mountpoint_path.join("bar.txt"),
                mountpoint_path.join("bar2.txt"),
            )
            .await;
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
        }
    );
}
