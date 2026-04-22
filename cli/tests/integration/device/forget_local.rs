use libparsec::{tmp_path, TmpPath};

use crate::{bootstrap_cli_test, testenv_utils::TestOrganization};

#[rstest::rstest]
#[tokio::test]
async fn forget_device(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

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
