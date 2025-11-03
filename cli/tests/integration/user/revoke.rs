use libparsec::{tmp_path, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

#[rstest::rstest]
#[tokio::test]
async fn revoke_user_ok(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, toto, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "user",
        "revoke",
        "--device",
        &alice.device_id.hex(),
        &toto.human_handle.email().to_string()
    )
    .stdout(predicates::str::contains(format!(
        "User {} has been revoked",
        toto.human_handle.email()
    )));
}

#[rstest::rstest]
#[tokio::test]
async fn revoke_user_not_found(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_failure!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "user",
        "revoke",
        "--device",
        &alice.device_id.hex(),
        "not-existing@example.com"
    )
    .stderr(predicates::str::contains("Error: User not found"));
}
