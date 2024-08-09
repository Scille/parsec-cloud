use libparsec::{tmp_path, TmpPath};

use super::bootstrap_cli_test;
use crate::testenv_utils::DEFAULT_DEVICE_PASSWORD;

#[rstest::rstest]
#[tokio::test]
async fn export_recovery_device(tmp_path: TmpPath) {
    let (_, [alice, ..], _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let output = tmp_path.join("recovery_device");

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "export-recovery-device",
        "--device",
        &alice.device_id.hex(),
        "--output",
        &output.to_string_lossy()
    )
    .stdout(predicates::str::contains("Saved in"));

    assert!(output.exists());
}
