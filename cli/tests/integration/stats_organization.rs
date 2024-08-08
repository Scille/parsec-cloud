use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::{get_testenv_config, run_local_organization, set_env};
use crate::testenv_utils::DEFAULT_ADMINISTRATION_TOKEN;

#[rstest::rstest]
#[tokio::test]
async fn stats_organization(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, org_id) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

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

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-organization",
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
