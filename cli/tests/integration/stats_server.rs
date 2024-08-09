use assert_cmd::Command;

use libparsec::{tmp_path, TmpPath};

use super::bootstrap_cli_test;
use crate::testenv_utils::DEFAULT_ADMINISTRATION_TOKEN;

#[rstest::rstest]
#[tokio::test]
// TODO: Split me into different tests
async fn stats_server(tmp_path: TmpPath) {
    let (url, _, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

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
