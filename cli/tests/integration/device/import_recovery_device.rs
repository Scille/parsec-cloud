use libparsec::{tmp_path, DeviceLabel, TmpPath};

use crate::{
    bootstrap_cli_test, test_ui,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::{ui::Ui, utils::start_client};

#[rstest::rstest]
#[case("password")]
#[case("keyring")]
#[tokio::test]
async fn import_recovery_device_password(tmp_path: TmpPath, test_ui: &Ui, #[case] auth: &str) {
    let (_, TestOrganization { alice, .. }, _) =
        bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    let input = tmp_path.join("recovery_device");

    let client = start_client(alice).await.unwrap();

    let (passphrase, data) = client
        .export_recovery_device(DeviceLabel::try_from("recovery".to_string().as_str()).unwrap())
        .await
        .unwrap();
    tokio::fs::write(&input, data).await.unwrap();

    crate::assert_cmd_success!(
        with_password = format!("{}\n{DEFAULT_DEVICE_PASSWORD}", *passphrase),
        "device",
        "import-recovery-device",
        "--input",
        &input,
        "--label",
        "new_device",
        "--auth",
        auth
    )
    .stderr(predicates::str::contains("New device created:"));
}
