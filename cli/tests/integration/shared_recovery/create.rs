use libparsec::{tmp_path, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

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
        &bob.human_handle.email().to_string(),
        &toto.human_handle.email().to_string(),
        "--weights",
        "1",
        "1",
        "--threshold",
        "1",
        "--no-confirmation"
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
        &bob.human_handle.email().to_string(),
        &toto.human_handle.email().to_string(),
        "--weights",
        "1",
        "--threshold",
        "1",
        "--no-confirmation"
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
        "1",
        "--no-confirmation"
    )
    .stderr(predicates::str::contains("A user is missing"));
}

#[cfg(target_family = "unix")] // rexpect doesn't support Windows
#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_default(tmp_path: TmpPath) {
    let (_, TestOrganization { bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();
    let cmd = crate::std_cmd!(
        "shared-recovery",
        "create",
        "--device",
        &bob.device_id.hex()
    );

    let mut p = crate::spawn_interactive_command(cmd, Some(1500)).unwrap();

    p.exp_string("Enter password for the device:").unwrap();
    p.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();

    // Wait for the intermediate spinners to make rexpect wait a bunch to prevent timeout when waiting for the "threshold" prompt
    p.exp_regex(".*Poll server for new certificates.*").unwrap();
    p.exp_regex(".*Creating shared recovery setup.*").unwrap();

    p.exp_regex(".*The threshold is the minimum number of recipients that one must gather to recover the account.*").unwrap();
    p.send_line("1").unwrap();
    p.exp_string("The following shared recovery setup will be created")
        .unwrap();
    p.send_line("y").unwrap();
    p.exp_regex(".*Shared recovery setup has been created.*")
        .unwrap();
    p.exp_eof().unwrap();
}

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_no_recipient(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    // Alice is the only admin
    crate::assert_cmd_failure!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "create",
        "--device",
        &alice.device_id.hex()
    )
    .stderr(predicates::str::contains("No default recipient available"));
}
