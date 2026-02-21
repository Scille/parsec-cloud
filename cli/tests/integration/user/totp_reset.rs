use libparsec::{tmp_path, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{DEFAULT_ADMINISTRATION_TOKEN, TESTBED_SERVER},
};

#[rstest::rstest]
#[tokio::test]
async fn totp_reset_by_email(tmp_path: TmpPath) {
    let (_, devices, org_id) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "user",
        "totp-reset",
        "--addr",
        &std::env::var(TESTBED_SERVER).unwrap(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--organization",
        org_id.as_ref(),
        "--user-email",
        &devices.alice.human_handle.email().to_string()
    )
    .stdout(predicates::str::contains("TOTP reset for user"))
    .stdout(predicates::str::is_match("Reset URL: .*parsec3://.*a=totp_reset&p=.*").unwrap());
}

#[rstest::rstest]
#[tokio::test]
async fn totp_reset_by_user_id(tmp_path: TmpPath) {
    let (_, devices, org_id) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "user",
        "totp-reset",
        "--addr",
        &std::env::var(TESTBED_SERVER).unwrap(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--organization",
        org_id.as_ref(),
        "--user-id",
        &devices.alice.user_id.hex(),
        "--send-email"
    )
    .stdout(predicates::str::contains("TOTP reset for user"))
    .stdout(predicates::str::is_match("Reset URL: .*parsec3://.*a=totp_reset&p=.*").unwrap())
    .stdout(
        predicates::str::is_match(
            "An email with the reset URL has been sent to .*alice@example.com",
        )
        .unwrap(),
    );
}

#[rstest::rstest]
#[tokio::test]
async fn totp_reset_user_not_found(tmp_path: TmpPath) {
    let (_, _, org_id) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_failure!(
        "user",
        "totp-reset",
        "--addr",
        &std::env::var(TESTBED_SERVER).unwrap(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--organization",
        org_id.as_ref(),
        "--user-email",
        "unknown@example.com"
    )
    .stderr(predicates::str::contains("User not found"));
}
