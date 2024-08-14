use std::io::{BufReader, Write};

use assert_cmd::cargo::CommandCargoExt;
use libparsec::{tmp_path, TmpPath};

use crate::{
    integration_tests::{bootstrap_cli_test, wait_for},
    testenv_utils::TestOrganization,
};

#[rstest::rstest]
#[tokio::test]
async fn remove_device(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let process = std::process::Command::cargo_bin("parsec-cli")
        .unwrap()
        .args(["device", "remove", "--device", &alice.device_id.hex()])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdout = BufReader::new(process.stdout.unwrap());
    let mut stdin = process.stdin.unwrap();
    let mut buf = String::new();

    let alice_device_file = tmp_path
        .join("config/parsec3/libparsec/devices")
        .join(format!("{}.keys", alice.device_id.hex()));

    assert!(alice_device_file.exists());

    wait_for(&mut stdout, &mut buf, "Are you sure? (y/n)");
    stdin.write_all(b"y\n").unwrap();

    wait_for(&mut stdout, &mut buf, "The device has been removed");

    assert!(!alice_device_file.exists());
}
