use libparsec::{tmp_path, TmpPath};
use parsec_cli::ui::Ui;

use crate::{bootstrap_cli_test, test_ui, testenv_utils::DEFAULT_ADMINISTRATION_TOKEN};

#[rstest::rstest]
#[tokio::test]
async fn freeze_by_email(tmp_path: TmpPath, test_ui: &Ui) {
    let (addr, devices, org_id) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "user",
        "freeze",
        "--addr",
        &addr.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--organization",
        org_id.as_ref(),
        "--user-email",
        &devices.alice.human_handle.email().to_string()
    )
    .stdout(
        predicates::str::is_match(format!(
            "User {uid} .* is frozen",
            uid = devices.alice.user_id.hex()
        ))
        .unwrap(),
    );

    crate::assert_cmd_success!(
        "user",
        "freeze",
        "--addr",
        &addr.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--organization",
        org_id.as_ref(),
        "--user-email",
        &devices.alice.human_handle.email().to_string(),
        "--unfreeze"
    )
    .stdout(
        predicates::str::is_match(format!(
            "User {uid} .* is not frozen",
            uid = devices.alice.user_id.hex()
        ))
        .unwrap(),
    );
}

#[rstest::rstest]
#[tokio::test]
async fn freeze_by_user_id(tmp_path: TmpPath, test_ui: &Ui) {
    let (addr, devices, org_id) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "user",
        "freeze",
        "--addr",
        &addr.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--organization",
        org_id.as_ref(),
        "--user-id",
        &devices.alice.user_id.hex()
    )
    .stdout(
        predicates::str::is_match(format!(
            "User {uid} .* is frozen",
            uid = devices.alice.user_id.hex()
        ))
        .unwrap(),
    );

    crate::assert_cmd_success!(
        "user",
        "freeze",
        "--addr",
        &addr.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--organization",
        org_id.as_ref(),
        "--user-id",
        &devices.alice.user_id.hex(),
        "--unfreeze"
    )
    .stdout(
        predicates::str::is_match(format!(
            "User {uid} .* is not frozen",
            uid = devices.alice.user_id.hex()
        ))
        .unwrap(),
    );
}

#[rstest::rstest]
#[tokio::test]
async fn freeze_user_not_found(tmp_path: TmpPath, test_ui: &Ui) {
    let (addr, _, org_id) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    crate::assert_cmd_failure!(
        "user",
        "freeze",
        "--addr",
        &addr.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--organization",
        org_id.as_ref(),
        "--user-email",
        "unknown@example.com"
    )
    .stderr(predicates::str::contains("User not found"));
}
