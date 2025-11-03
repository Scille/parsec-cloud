use std::io::{BufReader, Write};

use libparsec::{tmp_path, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
    wait_for,
};

#[rstest::rstest]
#[tokio::test]
async fn poll_and_forget(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "certificate",
        "poll",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("Added 7 new certificates"));

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "certificate",
        "poll",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("Added 0 new certificates"));

    let mut process = crate::std_cmd!(
        "certificate",
        "forget-all-certificates",
        "--device",
        &alice.device_id.hex(),
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
    stdin.write_all(b"y\n").unwrap();

    wait_for(
        &mut stdout,
        &mut buf,
        "The local certificates database has been cleared",
    );
    process.wait().unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "certificate",
        "poll",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("Added 7 new certificates"));
}
