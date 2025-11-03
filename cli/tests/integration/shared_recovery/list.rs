use libparsec::{tmp_path, TmpPath};

use crate::{
    bootstrap_cli_test, shared_recovery_create,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::utils::{GREEN, RESET};

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

    shared_recovery_create(&alice, &bob, Some(&toto));

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "list",
        "--device",
        &bob.device_id.hex()
    )
    .stdout(predicates::str::contains(format!(
        "Shared recovery for {GREEN}{}{RESET}",
        alice.human_handle
    )));
}
