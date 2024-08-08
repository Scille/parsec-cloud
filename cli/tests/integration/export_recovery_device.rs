use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::bootstrap_cli_test;
use crate::testenv_utils::DEFAULT_DEVICE_PASSWORD;

#[rstest::rstest]
#[tokio::test]
async fn export_recovery_device(tmp_path: TmpPath) {
    let (_, [alice, ..], _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let output = tmp_path.join("recovery_device");

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "export-recovery-device",
            "--device",
            &alice.device_id.hex(),
            "--output",
            &output.to_string_lossy(),
            "--password-stdin",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .stdout(predicates::str::contains("Saved in"));

    assert!(output.exists());
}
