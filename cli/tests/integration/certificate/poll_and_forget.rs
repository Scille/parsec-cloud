use libparsec::{tmp_path, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
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

    let cmd = crate::std_cmd!(
        "certificate",
        "forget-all-certificates",
        "--device",
        &alice.device_id.hex(),
        "--password-stdin"
    );
    let mut p = crate::spawn_interactive_command(cmd, Some(1500)).unwrap();

    p.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
    p.exp_regex(".*Are you sure?.*").unwrap();
    p.send_line("y").unwrap();

    p.exp_string("The local certificates database has been cleared")
        .unwrap();
    p.exp_eof().unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "certificate",
        "poll",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("Added 7 new certificates"));
}
