use std::io::{BufReader, Write};

use libparsec::{tmp_path, TmpPath};

use crate::{bootstrap_cli_test, testenv_utils::TestOrganization, wait_for};

#[rstest::rstest]
#[tokio::test]
async fn remove_device(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let mut process = crate::std_cmd!("device", "remove", "--device", &alice.device_id.hex())
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout = BufReader::new(process.stdout.as_mut().unwrap());
    let stdin = process.stdin.as_mut().unwrap();
    let mut buf = String::new();

    let alice_device_file = tmp_path
        .join("config/parsec3/libparsec/devices")
        .join(format!("{}.keys", alice.device_id.hex()));

    assert!(alice_device_file.exists());

    wait_for(&mut stdout, &mut buf, "Are you sure? (y/n)");
    stdin.write_all(b"y\n").unwrap();

    wait_for(&mut stdout, &mut buf, "The device has been removed");
    process.wait().unwrap();

    assert!(!alice_device_file.exists());
}
