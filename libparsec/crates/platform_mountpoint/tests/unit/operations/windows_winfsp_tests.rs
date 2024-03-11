// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![cfg(target_os = "windows")]

use std::{path::PathBuf, process::Command, sync::Arc};

use libparsec_client::{Client, WorkspaceOps};
use libparsec_tests_fixtures::prelude::*;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    let winfsp_test_exe =
        std::env::var("WINFSP_TEST_EXE").expect("`WINFSP_TEST_EXE` environ variable must be set");

    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            tokio::task::spawn_blocking(move || {
                let mut tests = Command::new(winfsp_test_exe)
                    .args([
                        // cspell: disable
                        "--external",
                        "--resilient",
                        // Require a case-insensitive file system
                        "-getfileinfo_name_test",
                        // Reparse point are not supported
                        "-reparse_guid_test",
                        "-reparse_nfs_test",
                        "-reparse_symlink_test",
                        "-reparse_symlink_relative_test",
                        // Stream are not supported
                        "-stream_*",
                        // Tests failing on file systems mounted as directories
                        "-create_test",
                        "-getfileinfo_test",
                        "-getfileinfo_name_test",
                        "-getvolinfo_test",
                        // Setting file attributes is not supported at the moment
                        "-create_fileattr_test",
                        "-create_readonlydir_test",
                        "-getfileattr_test",
                        "-setfileinfo_test",
                        "-delete_access_test",
                        // Setting allocation size is not supported at the moment
                        "-create_allocation_test",
                        // Renaming has a special behavior in parsec
                        "-rename_test",
                        "-rename_mmap_test",
                        "-rename_standby_test",
                        "-exec_rename_test",
                        // Setting security is not supported at the moment
                        "-getsecurity_test",
                        "-setsecurity_test",
                        // Incompatibility between Parsec entry name UTF8 limit in bytes and WinFSP WCHAR UTF16 limit
                        "-create_namelen_test",
                        "-querydir_namelen_test",
                        // TODO: investigate why this test only fails in CI
                        "-create_backup_test",
                        "-create_restore_test",
                        "-create_related_test",
                        // TODO: investigate misc tests
                        "-create_notraverse_test",
                        "-delete_mmap_test",
                        "-delete_ex_test",
                        "-rdwr_noncached_overlapped_test",
                        "-rdwr_cached_overlapped_test",
                        "-rdwr_writethru_overlapped_test",
                        "-lock_noncached_overlapped_test",
                        "-lock_cached_overlapped_test",
                        // TODO: rename is currently broken in Parsec
                        "-rename_ex_test",
                        // cspell: enable
                    ])
                    .current_dir(&mountpoint_path)
                    .spawn()
                    .expect("Can't run winfsp-tests");

                let code = tests.wait().expect("Tests should exit");
                assert!(code.success());
            })
            .await
            .unwrap();
        }
    )
}
