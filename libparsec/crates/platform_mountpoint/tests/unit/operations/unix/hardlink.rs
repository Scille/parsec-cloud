// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::WorkspaceOps;
use libparsec_tests_fixtures::{tmp_path, TestbedEnv, TmpPath};
use libparsec_tests_lite::{p_assert_matches, parsec_test};

use crate::operations::utils::mount_and_test;

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
            p_assert_matches!(result, Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied);
        }
    );
}
