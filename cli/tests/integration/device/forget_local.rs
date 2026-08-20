#[cfg(target_family = "unix")]
use libparsec::{tmp_path, TmpPath};
#[cfg(target_family = "unix")]
use parsec_cli::ui::Ui;

#[cfg(target_family = "unix")]
use crate::{bootstrap_cli_test, test_ui, testenv_utils::TestOrganization};

#[cfg(target_family = "unix")] // rexpect doesn't support Windows
#[rstest::rstest]
#[tokio::test]
async fn forget_device(tmp_path: TmpPath, test_ui: &Ui) {
    let (_, TestOrganization { alice, .. }, _) =
        bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    let cmd = crate::std_cmd!("device", "forget-local", "--device", &alice.device_id.hex());
    let mut p = crate::spawn_interactive_command(cmd, Some(1500)).unwrap();

    let alice_device_file = tmp_path
        .join("config/parsec3/libparsec/devices")
        .join(format!("{}.keys", alice.device_id.hex()));

    assert!(alice_device_file.exists());

    p.exp_regex(".*Are you sure?.*").unwrap();
    p.send_line("y").unwrap();

    p.exp_string("The local device has been forgotten").unwrap();
    p.exp_eof().unwrap();

    assert!(!alice_device_file.exists());
}

#[rstest::rstest]
#[tokio::test]
async fn forget_device_force(tmp_path: TmpPath, test_ui: &Ui) {
    let (_, TestOrganization { alice, .. }, _) =
        bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    let alice_device_file = tmp_path
        .join("config/parsec3/libparsec/devices")
        .join(format!("{}.keys", alice.device_id.hex()));

    assert!(alice_device_file.exists());
    crate::assert_cmd_success!(
        "device",
        "forget-local",
        "--device",
        alice.device_id.hex(),
        "--force"
    )
    .stderr(predicates::str::contains(
        "The local device has been forgotten",
    ));

    assert!(!alice_device_file.exists());
}
