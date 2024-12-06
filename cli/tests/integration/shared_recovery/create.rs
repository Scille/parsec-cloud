use libparsec::{tmp_path, TmpPath};

use crate::integration_tests::bootstrap_cli_test;
use crate::testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD};
use rexpect::session::spawn;

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_ok(tmp_path: TmpPath) {
    let (
        _,
        TestOrganization {
            alice, bob, toto, ..
        },
        _,
    ) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "create",
        "--device",
        &alice.device_id.hex(),
        "--recipients",
        &bob.human_handle.email(),
        &toto.human_handle.email(),
        "--weights",
        "1",
        "1",
        "--threshold",
        "1"
    )
    .stdout(predicates::str::contains(
        "Shared recovery setup has been created",
    ));
}

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_incoherent_weights(tmp_path: TmpPath) {
    let (
        _,
        TestOrganization {
            alice, bob, toto, ..
        },
        _,
    ) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_failure!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "create",
        "--device",
        &alice.device_id.hex(),
        "--recipients",
        &bob.human_handle.email(),
        &toto.human_handle.email(),
        "--weights",
        "1",
        "--threshold",
        "1"
    )
    .stderr(predicates::str::contains("incoherent weights count"));
}

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_inexistent_email(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();
    // a non existent recipient

    crate::assert_cmd_failure!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "create",
        "--device",
        &alice.device_id.hex(),
        "--recipients",
        "not-here@example.com",
        "--weights",
        "1",
        "--threshold",
        "1"
    )
    .stderr(predicates::str::contains("A user is missing"));
}

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_default(tmp_path: TmpPath) {
    let (_, TestOrganization { bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();
    let mut cmd = assert_cmd::Command::cargo_bin("parsec-cli").unwrap();
    cmd.args([
        "shared-recovery",
        "create",
        "--device",
        &bob.device_id.hex(),
    ]);

    let program = cmd.get_program().to_str().unwrap().to_string();
    let program = cmd
        .get_args()
        .fold(program, |acc, s| format!("{acc} {s:?}"));

    let mut p = spawn(&dbg!(program), Some(1000)).unwrap();

    p.exp_string("Enter password for the device:").unwrap();
    p.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();

    p.exp_regex(".*The threshold is the minimum number of recipients that one must gather to recover the account:.*").unwrap();
    p.send_line("1").unwrap();
    p.exp_regex(".*Shared recovery setup has been created.*")
        .unwrap();
    p.exp_eof().unwrap();
}
