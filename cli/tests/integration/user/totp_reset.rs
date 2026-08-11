use libparsec::{tmp_path, TmpPath};
use parsec_cli::ui::Ui;

use crate::{
    bootstrap_cli_test, test_ui,
    testenv_utils::{DEFAULT_ADMINISTRATION_TOKEN, TESTBED_SERVER},
};

#[rstest::rstest]
#[tokio::test]
async fn totp_reset_by_email(tmp_path: TmpPath, test_ui: &Ui) {
    let (_, devices, org_id) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

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
    .stdout(
        predicates::str::is_match(format!(
            "TOTP reset URL for user {uid} is parsec3://.*a=totp_reset&p=.*",
            uid = devices.alice.user_id.hex()
        ))
        .unwrap(),
    );
}

#[rstest::rstest]
#[tokio::test]
async fn totp_reset_by_user_id(tmp_path: TmpPath, test_ui: &Ui) {
    let (_, devices, org_id) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

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
    .stdout(
        predicates::str::is_match(format!(
            "TOTP reset URL for user {uid} is parsec3://.*a=totp_reset&p=.*",
            uid = devices.alice.user_id.hex()
        ))
        .unwrap(),
    )
    .stderr(predicates::str::contains(format!(
        "An email with the reset URL has been sent to {}",
        devices.alice.human_handle.email()
    )));
}

#[rstest::rstest]
#[tokio::test]
async fn totp_reset_user_not_found(tmp_path: TmpPath, test_ui: &Ui) {
    let (_, _, org_id) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

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
