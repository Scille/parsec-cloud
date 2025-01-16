// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::WorkspaceOps;
use libparsec_tests_fixtures::{rstest, tmp_path, TestbedEnv, TmpPath};
use libparsec_tests_lite::{p_assert_matches, parsec_test};

use crate::operations::utils::mount_and_test;

// NOTE:
// 1. We cannot create link in the parsec mountpoint
//    So we can only do readlink on file/folder.
// 2. If you inspect the event log of the mountpoint, you will see that readlink is not called.
//    The reason is fuse has a pre-check that fail early if it's not a link.

#[parsec_test(testbed = "minimal_client_ready")]
#[case::file("bar.txt")]
#[case::folder("foo")]
async fn readlink_on(#[case] file: &str, tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        async |_client, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| {
            let file_path = mountpoint_path.join(file);

            let res = tokio::fs::read_link(file_path).await;
            p_assert_matches!(res, Err(err) if err.kind() == std::io::ErrorKind::InvalidInput);
        }
    );
}
