use std::io::{BufReader, Write};

use libparsec::{tmp_path, ParsecAddr, TmpPath};
use libparsec_tests_fixtures::prelude::*;

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
    wait_for,
};
use parsec_cli::utils::YELLOW;

#[rstest::rstest]
#[tokio::test]
async fn ok(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let mut process = crate::std_cmd!(
        "device",
        "overwrite-server-url",
        "--device",
        &alice.device_id.hex(),
        "--server-url",
        "https://new.invalid:123",
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

    let mut buf = String::new();
    wait_for(&mut stdout, &mut buf, "Are you sure? (y/n)");

    let old_server_url = ParsecAddr::new(
        alice.organization_addr.hostname().to_owned(),
        Some(alice.organization_addr.port()),
        alice.organization_addr.use_ssl(),
    )
    .to_http_url(None);
    assert!(
        buf.contains(&format!("Current server URL: {YELLOW}{old_server_url}")),
        "stdout: {buf}"
    );
    assert!(
        buf.contains(&format!("New server URL: {YELLOW}https://new.invalid:123/")),
        "stdout: {buf}"
    );

    stdin.write_all(b"y\n").unwrap();

    wait_for(&mut stdout, &mut buf, "Device updated successfully");
    process.wait().unwrap();

    // TODO: Inspect the device content once `device info` CLI command is available
}
