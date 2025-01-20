use libparsec::{tmp_path, TmpPath};

use crate::testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD};
use crate::utils::*;

use crate::integration_tests::bootstrap_cli_test;

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
        "1",
        "--no-confirmation"
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
