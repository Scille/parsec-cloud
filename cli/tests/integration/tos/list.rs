use libparsec::{tmp_path, TmpPath};

use crate::{
    integration_tests::bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

#[rstest::rstest]
#[tokio::test]
async fn list_no_tos_available(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "tos",
        "list",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("No Terms of Service available"));
}

// FIXME: How to configure the TOS using the testbed server ?
#[should_panic]
#[rstest::rstest]
#[tokio::test]
async fn list_tos_ok(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "tos",
        "list",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains(
        "Terms of Service updated on __TODO__:",
    ));
}
