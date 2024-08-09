use libparsec::{tmp_path, TmpPath};

use super::bootstrap_cli_test;
use crate::testenv_utils::DEFAULT_DEVICE_PASSWORD;

#[rstest::rstest]
#[tokio::test]
async fn create_workspace(tmp_path: TmpPath) {
    let (_, [alice, ..], _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "create-workspace",
        "--device",
        &alice.device_id.hex(),
        "--name",
        "new-workspace"
    )
    .stdout(predicates::str::contains("Workspace has been created"));

    // TODO: Replace with client.list_workspaces directly instead of using the CLI
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "list-workspaces",
        "--device",
        &alice.device_id.hex()
    )
    .stdout(predicates::str::contains("new-workspace: owner"));
}
