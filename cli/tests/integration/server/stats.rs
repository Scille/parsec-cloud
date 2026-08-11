use libparsec::{tmp_path, TmpPath};
use parsec_cli::ui::Ui;
use serde_json::Value;

use crate::{bootstrap_cli_test, test_ui, testenv_utils::DEFAULT_ADMINISTRATION_TOKEN};

#[rstest::rstest]
#[tokio::test]
async fn stats_server(tmp_path: TmpPath, test_ui: &Ui) {
    let (url, _, _) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    let result = crate::assert_cmd_success!(
        "--format=json",
        "server",
        "stats",
        "--addr",
        &url.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN
    );
    let output = result.get_output();

    // NOTE: Not check the parsed value as it's not consistent as the test server is likely used by
    // other tests adding some "garbage" data.
    serde_json::from_slice::<Value>(&output.stdout).expect("The command returned invalid JSON");
}

#[rstest::rstest]
#[tokio::test]
async fn csv_format(tmp_path: TmpPath, test_ui: &Ui) {
    let (url, _, _) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        "--format=plain", // CSV format is obtained by specifying the plain format
        "server",
        "stats",
        "--addr",
        &url.to_string(),
        "--token",
        DEFAULT_ADMINISTRATION_TOKEN
    )
        // NOTE: Only check the CSV header for the same reason as for JSON output.
        .stdout(predicates::str::contains("organization_id,data_size,metadata_size,realms,active_users,admin_users_active,admin_users_revoked,standard_users_active,standard_users_revoked,outsider_users_active,outsider_users_revoked\r\n"));
}

#[rstest::rstest]
#[tokio::test]
async fn with_end_date(tmp_path: TmpPath, test_ui: &Ui) {
    let (url, _, _) = bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

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
