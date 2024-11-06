use libparsec::{tmp_path, DeviceLabel, TmpPath};

use crate::{
    integration_tests::bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
    utils::start_client,
};

#[rstest::rstest]
#[tokio::test]
async fn import_recovery_device(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let input = tmp_path.join("recovery_device");

    let client = start_client(alice).await.unwrap();

    let (passphrase, data) = client
        .client_export_recovery_device(
            DeviceLabel::try_from("recovery".to_string().as_str()).unwrap(),
        )
        .await
        .unwrap();
    tokio::fs::write(&input, data).await.unwrap();

    crate::assert_cmd_success!(
        with_password = format!("{}\n{DEFAULT_DEVICE_PASSWORD}", *passphrase),
        "device",
        "import-recovery-device",
        "--input",
        &input.to_string_lossy(),
        "--label",
        "new_device"
    )
    .stdout(predicates::str::contains("Saved new device"));
}
