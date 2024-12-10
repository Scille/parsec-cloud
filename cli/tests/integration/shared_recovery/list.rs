use libparsec::{tmp_path, TmpPath};

use crate::integration_tests::shared_recovery::setup_shared_recovery;
use crate::testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD};
use crate::utils::*;

use crate::integration_tests::bootstrap_cli_test;

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

    setup_shared_recovery(&alice, &bob, &toto);

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
