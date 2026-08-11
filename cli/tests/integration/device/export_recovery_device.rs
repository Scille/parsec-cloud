use libparsec::{tmp_path, TmpPath};
use parsec_cli::ui::Ui;

use crate::{
    bootstrap_cli_test, test_ui,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

#[rstest::rstest]
#[tokio::test]
async fn export_recovery_device(tmp_path: TmpPath, test_ui: &Ui) {
    let (_, TestOrganization { alice, .. }, _) =
        bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    let output = tmp_path.join("recovery_device");

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "device",
        "export-recovery-device",
        "--device",
        alice.device_id.hex(),
        &output
    )
    .stdout(predicates::str::contains("Recovery device saved at"));

    assert!(output.exists());
}
