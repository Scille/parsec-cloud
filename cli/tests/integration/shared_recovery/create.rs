use libparsec::{tmp_path, TmpPath};

use crate::testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD};

use crate::integration_tests::bootstrap_cli_test;

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
