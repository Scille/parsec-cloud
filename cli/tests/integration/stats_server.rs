use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::{get_testenv_config, run_local_organization, set_env};
use crate::testenv_utils::DEFAULT_ADMINISTRATION_TOKEN;

#[rstest::rstest]
#[tokio::test]
// TODO: Split me into different tests
async fn stats_server(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, _, _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-server",
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
        ])
        .unwrap();

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-server",
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
            "--format",
            "csv",
        ])
        .assert()
        .stdout(predicates::str::contains("organization_id,data_size,metadata_size,realms,active_users,admin_users_active,admin_users_revoked,standard_users_active,standard_users_revoked,outsider_users_active,outsider_users_revoked\r\n"));

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "stats-server",
            "--addr",
            &url.to_string(),
            "--token",
            DEFAULT_ADMINISTRATION_TOKEN,
            "--end-date",
            "1990-01-01T00:00:00-00:00",
        ])
        .unwrap();
}
