use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::bootstrap_cli_test;
use crate::testenv_utils::DEFAULT_ADMINISTRATION_TOKEN;

#[rstest::rstest]
#[tokio::test]
async fn status_organization(tmp_path: TmpPath) {
    let (url, _, org_id) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let expect = format!(
        "{:#}\n",
        serde_json::json!({
            "active_users_limit": null,
            "is_bootstrapped": true,
            "is_expired": false,
            "minimum_archiving_period": 2592000,
            "user_profile_outsider_allowed": true
        })
    );

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "status-organization",
            "--organization-id",
            org_id.as_ref(),
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
        ])
        .assert()
        .stdout(expect);
}
