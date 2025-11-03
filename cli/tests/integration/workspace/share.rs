use libparsec::{tmp_path, RealmRole, TmpPath};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::utils::start_client;

#[rstest::rstest]
#[tokio::test]
async fn share_workspace(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let wid = {
        log::debug!("Create a workspace for alice");
        let alice_client = start_client(alice.clone()).await.unwrap();

        let wid = alice_client
            .create_workspace("new-workspace".parse().unwrap())
            .await
            .unwrap();
        log::trace!("Workspace ID: {wid}");

        alice_client.ensure_workspaces_bootstrapped().await.unwrap();

        alice_client
            .poll_server_for_new_certificates()
            .await
            .unwrap();

        alice_client.stop().await;

        wid
    };

    log::debug!("Share the workspace with bob as a contributor");
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "share",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        "--user",
        &bob.user_id.hex(),
        "--role",
        "contributor"
    )
    .stdout(predicates::str::contains("Workspace has been shared"));

    log::debug!("Check if bob has been added to the workspace as a contributor");
    let bob_client = start_client(bob).await.unwrap();

    bob_client.poll_server_for_new_certificates().await.unwrap();
    bob_client.refresh_workspaces_list().await.unwrap();
    let workspaces = bob_client.list_workspaces().await;

    let workspace_name = "new-workspace".parse().unwrap();
    assert!(
        workspaces
            .iter()
            .any(|w| w.current_name == workspace_name
                && w.current_self_role == RealmRole::Contributor),
        "Missing shared workspace for bob, {workspaces:?}"
    );
}
