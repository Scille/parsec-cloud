use libparsec::{tmp_path, RealmRole, TmpPath};

use crate::tests::bootstrap_cli_test;
use crate::{
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
    utils::start_client,
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
        "--name",
        "new-workspace"
    )
    .stdout(predicates::str::contains("Workspace has been created"));

    let client = start_client(alice).await.unwrap();

    let workspaces = client.list_workspaces().await;

    let workspace_name = "new-workspace".parse().unwrap();
    assert!(workspaces
        .iter()
        .any(|w| w.current_name == workspace_name && w.current_self_role == RealmRole::Owner));
}
