use libparsec::{tmp_path, TmpPath};

use crate::testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD};

use super::bootstrap_cli_test;

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery(tmp_path: TmpPath) {
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
        "--threshold",
        "1"
    )
    .stdout(predicates::str::contains(
        "Shared recovery setup has been created",
    ));
    // TODO list shamir setup
}
