use libparsec::{tmp_path, TmpPath};

use crate::{bootstrap_cli_test, testenv_utils::DEFAULT_ADMINISTRATION_TOKEN};

#[rstest::rstest]
#[tokio::test]
async fn stats_organization(tmp_path: TmpPath) {
    let (url, _, org_id) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let expect = format!(
        "{:#}\n",
        serde_json::json!({
            "active_users": 3,
            "data_size": 0,
            "metadata_size": 0,
            "realms": 0,
            "users": 3,
            "users_per_profile_detail": {
                "ADMIN": {
                    "active": 1,
                    "revoked": 0
                },
                "STANDARD": {
                    "active": 1,
                    "revoked": 0
                },
                "OUTSIDER": {
                    "active": 1,
                    "revoked": 0
                }
            }
        })
    );

    crate::assert_cmd_success!(
        "organization",
        "stats",
        "--organization",
        org_id.as_ref(),
        "--addr",
        &url.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN
    )
    .stdout(expect);
}
