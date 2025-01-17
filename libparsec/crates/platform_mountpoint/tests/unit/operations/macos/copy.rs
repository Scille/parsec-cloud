use std::path::PathBuf;

use libparsec_tests_fixtures::{tmp_path, TestbedEnv, TmpPath};
use libparsec_tests_lite::parsec_test;

use crate::operations::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn copy_file_using_finder(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client, _wksp_ops, mountpoint_path: PathBuf| async move {
            const FILE_CONTENT: &str = "I'm the file content that should be copied";
            let src_file = tmp_path.join("source.txt");
            let dst_file = mountpoint_path.join("source.txt");
            let script_path = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
                .join("scripts/macos/copy-using-finder.scpt");

            assert!(
                !tokio::fs::try_exists(&dst_file).await.unwrap(),
                "The destination file should not exist before the copy"
            );

            // Create the source file with some content
            tokio::fs::write(&src_file, FILE_CONTENT).await.unwrap();

            // Call the osascript that perform the copy using Finder.app
            let status = tokio::process::Command::new("osascript")
                .args([&script_path, &src_file, &mountpoint_path])
                .status()
                .await
                .unwrap();
            assert!(status.success());

            // Verify the content of the copied file
            let data = tokio::fs::read_to_string(&dst_file).await.unwrap();
            assert_eq!(data, FILE_CONTENT);
        }
    );
}
