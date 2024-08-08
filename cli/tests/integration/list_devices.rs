use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::bootstrap_cli_test;
use crate::utils::{GREEN, RESET, YELLOW};

#[rstest::rstest]
#[tokio::test]
async fn list_devices(tmp_path: TmpPath) {
    bootstrap_cli_test(&tmp_path).await.unwrap();

    let path = tmp_path.join("config/parsec3/libparsec");
    let path_str = path.to_string_lossy();

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .arg("list-devices")
        .assert()
        .stdout(predicates::str::contains(format!(
            "Found {GREEN}4{RESET} device(s) in {YELLOW}{path_str}{RESET}:"
        )));
}
