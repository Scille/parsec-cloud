use libparsec::{tmp_path, RealmRole, TmpPath};

use crate::{
    integration_tests::bootstrap_cli_test,
    testenv_utils::{
        client_config_without_monitors_running, TestOrganization, DEFAULT_DEVICE_PASSWORD,
    },
    utils::start_client_with_config,
};

#[rstest::rstest]
#[tokio::test]
async fn create_workspace(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "create",
        "--device",
        &alice.device_id.hex(),
        "new-workspace"
    )
    .stdout(predicates::str::contains("Workspace has been created"));

    let client = start_client_with_config(alice, client_config_without_monitors_running())
        .await
        .unwrap();

    let workspaces = client.list_workspaces().await;

    let workspace_name = "new-workspace".parse().unwrap();
    assert!(workspaces
        .iter()
        .any(|w| w.current_name == workspace_name && w.current_self_role == RealmRole::Owner));
}
