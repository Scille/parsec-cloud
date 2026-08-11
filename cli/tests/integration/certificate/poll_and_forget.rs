#[cfg(target_family = "unix")]
use libparsec::{tmp_path, TmpPath};
#[cfg(target_family = "unix")]
use parsec_cli::ui::Ui;

#[cfg(target_family = "unix")]
use crate::{
    bootstrap_cli_test, test_ui,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

#[cfg(target_family = "unix")] // rexpect doesn't support Windows
#[rstest::rstest]
#[tokio::test]
async fn poll_and_forget(tmp_path: TmpPath, test_ui: &Ui) {
    let (_, TestOrganization { alice, .. }, _) =
        bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "certificate",
        "poll",
        "--device",
        &alice.device_id.hex()
    )
    .stderr(predicates::str::contains("Added 7 new certificates"));

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "certificate",
        "poll",
        "--device",
        &alice.device_id.hex()
    )
    .stderr(predicates::str::contains("Added 0 new certificates"));

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
    .stderr(predicates::str::contains("Added 7 new certificates"));
}
