use libparsec::{tmp_path, TmpPath};

use crate::testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD};
use crate::utils::*;

use super::bootstrap_cli_test;

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
async fn list_shared_recovery_ok(tmp_path: TmpPath) {
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
        "list",
        "--device",
        &bob.device_id.hex()
    )
    .stdout(predicates::str::contains("No shared recovery found"));

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

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "list",
        "--device",
        &bob.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "Shared recovery for {GREEN}{}{RESET}",
        alice.user_id
    )));
}

#[rstest::rstest]
#[tokio::test]
async fn info_shared_recovery_ok(tmp_path: TmpPath) {
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
        "info",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "Shared recovery {RED}never setup{RESET}"
    )));
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

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "info",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "Shared recovery {GREEN}set up{RESET}"
    )));
}

#[rstest::rstest]
#[tokio::test]
async fn remove_shared_recovery_ok(tmp_path: TmpPath) {
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
        "info",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "Shared recovery {RED}never setup{RESET}"
    )));
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

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "info",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "Shared recovery {GREEN}set up{RESET}"
    )));

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "delete",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "Deleted shared recovery for {}",
        alice.user_id
    )));

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "info",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "{RED}Deleted{RESET} shared recovery"
    )));
}
