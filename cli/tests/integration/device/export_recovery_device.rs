use libparsec::{tmp_path, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

#[rstest::rstest]
#[tokio::test]
async fn export_recovery_device(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let output = tmp_path.join("recovery_device");

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "device",
        "export-recovery-device",
        "--device",
        &alice.device_id.hex(),
        &output.to_string_lossy()
    )
    .stdout(predicates::str::contains("Recovery device saved at"));

    assert!(output.exists());
}
