use libparsec::{tmp_path, EntryName, RealmArchivingConfiguration, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::utils::start_client;

#[rstest::rstest]
#[tokio::test]
async fn archive_workspace(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let (wid, workspace_name) = {
        let alice_client = start_client(alice.clone()).await.unwrap();

        let workspace_name: EntryName = "new-workspace".parse().unwrap();
        let wid = alice_client
            .create_workspace(workspace_name.clone())
            .await
            .unwrap();

        alice_client.ensure_workspaces_bootstrapped().await.unwrap();
        alice_client
            .poll_server_for_new_certificates()
            .await
            .unwrap();
        alice_client.refresh_workspaces_list().await.unwrap();
        alice_client.stop().await;

        (wid, workspace_name)
    };

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "archive",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        "--archived"
    )
    .stdout(predicates::str::contains(
        "Workspace archiving status has been updated",
    ));

    let alice_client = start_client(alice).await.unwrap();
    alice_client
        .poll_server_for_new_certificates()
        .await
        .unwrap();
    alice_client.refresh_workspaces_list().await.unwrap();
    let workspaces = alice_client.list_workspaces().await;

    let workspace = workspaces
        .iter()
        .find(|w| w.id == wid)
        .expect("workspace should exist");
    assert_eq!(workspace.name, workspace_name);
    assert_eq!(
        workspace.archiving_configuration,
        RealmArchivingConfiguration::Archived
    );

    alice_client.stop().await;
}
