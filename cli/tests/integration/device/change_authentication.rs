use std::io::{BufReader, Write};

use libparsec::{tmp_path, TmpPath};
use libparsec_tests_fixtures::prelude::*;

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
    wait_for,
};

#[rstest::rstest]
#[tokio::test]
async fn ok(tmp_path: TmpPath) {
    const NEW_DEVICE_PASSWORD: &str = "S3cr3t";

    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let mut process = crate::std_cmd!(
        "device",
        "change-authentication",
        "--device",
        &alice.device_id.hex(),
        "--password",
        "--password-stdin"
    )
    .stdin(std::process::Stdio::piped())
    .stderr(std::process::Stdio::inherit())
    .stdout(std::process::Stdio::piped())
    .spawn()
    .unwrap();

    let mut stdout = BufReader::new(process.stdout.as_mut().unwrap());
    let stdin = process.stdin.as_mut().unwrap();

    stdin.write_all(DEFAULT_DEVICE_PASSWORD.as_bytes()).unwrap();
    stdin.write_all(b"\n").unwrap();

    stdin.write_all(NEW_DEVICE_PASSWORD.as_bytes()).unwrap();
    stdin.write_all(b"\n").unwrap();

    let mut buf = String::new();
    wait_for(
        &mut stdout,
        &mut buf,
        "Device authentication changed successfully",
    );
    process.wait().unwrap();

    // Now ensure we can authenticate with the new password
    crate::assert_cmd_success!(
        with_password = NEW_DEVICE_PASSWORD,
        "workspace",
        "list",
        "--device",
        &alice.device_id.hex()
    );
}
