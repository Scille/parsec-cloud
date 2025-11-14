use libparsec::{tmp_path, TmpPath};

use crate::{bootstrap_cli_test, testenv_utils::DEFAULT_ADMINISTRATION_TOKEN};

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
            "user_profile_outsider_allowed": true,
            "tos": null
        })
    );

    crate::assert_cmd_success!(
        "organization",
        "status",
        "--organization",
        org_id.as_ref(),
        "--addr",
        &url.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN
    )
    .stdout(expect);
}
