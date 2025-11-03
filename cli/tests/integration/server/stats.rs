use libparsec::{tmp_path, TmpPath};

use crate::{bootstrap_cli_test, testenv_utils::DEFAULT_ADMINISTRATION_TOKEN};

#[rstest::rstest]
#[tokio::test]
async fn stats_server(tmp_path: TmpPath) {
    let (url, _, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "server",
        "stats",
        "--addr",
        &url.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN
    );
}

#[rstest::rstest]
#[tokio::test]
async fn csv_format(tmp_path: TmpPath) {
    let (url, _, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "server",
        "stats",
        "--addr",
        &url.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--format",
        "csv"
    )
        .stdout(predicates::str::contains("organization_id,data_size,metadata_size,realms,active_users,admin_users_active,admin_users_revoked,standard_users_active,standard_users_revoked,outsider_users_active,outsider_users_revoked\r\n"));
}

#[rstest::rstest]
#[tokio::test]
async fn with_end_date(tmp_path: TmpPath) {
    let (url, _, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "server",
        "stats",
        "--addr",
        &url.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN,
        "--end-date",
        "1990-01-01T00:00:00-00:00"
    );
}
