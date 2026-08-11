use libparsec::{tmp_path, TmpPath};

use crate::{
    bootstrap_cli_test, shared_recovery_create,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

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

    shared_recovery_create(&alice, &bob, Some(&toto));

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "delete",
        "--device",
        &alice.device_id.hex()
    )
    .stderr(predicates::str::contains(
        "Shared recovery setup has been deleted",
    ));

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "info",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("Deleted shared recovery"));
}
