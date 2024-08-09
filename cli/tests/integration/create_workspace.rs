use std::sync::Arc;

use libparsec::{tmp_path, RealmRole, TmpPath};

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

    let client = libparsec::internal::Client::start(
        Arc::new(
            libparsec::ClientConfig {
                with_monitors: false,
                ..Default::default()
            }
            .into(),
        ),
        libparsec::internal::EventBus::default(),
        alice.clone(),
    )
    .await
    .unwrap();

    let workspaces = client.list_workspaces().await;

    let workspace_name = "new-workspace".parse().unwrap();
    assert!(workspaces
        .iter()
        .any(|w| w.current_name == workspace_name && w.current_self_role == RealmRole::Owner));
}
