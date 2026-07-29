use libparsec::{tmp_path, TmpPath};

use crate::bootstrap_cli_test;

#[rstest::rstest]
#[tokio::test]
async fn list_devices(tmp_path: TmpPath) {
    bootstrap_cli_test(&tmp_path).await.unwrap();

    let path = tmp_path.join("config/parsec3/libparsec");
    let path_str = path.to_string_lossy();

    crate::assert_cmd_success!("device", "list").stderr(predicates::str::contains(format!(
        "Found 4 device(s) in {path_str}"
    )));
}
