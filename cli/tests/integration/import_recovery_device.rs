use libparsec::{tmp_path, TmpPath};

use super::bootstrap_cli_test;
use crate::testenv_utils::DEFAULT_DEVICE_PASSWORD;

#[rstest::rstest]
#[tokio::test]
async fn import_recovery_device(tmp_path: TmpPath) {
    let (_, [alice, ..], _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let input = tmp_path.join("recovery_device");

    let passphrase = libparsec::save_recovery_device(&input, &alice)
        .await
        .unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "import-recovery-device",
        "--input",
        &input.to_string_lossy(),
        "--passphrase",
        &passphrase
    )
    .stdout(predicates::str::contains("Saved new device"));
}
