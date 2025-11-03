use std::sync::Arc;

use libparsec::{internal::Client, tmp_path, EntryName, EntryStat, LocalDevice, TmpPath, VlobID};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::utils::{start_client, StartedClient};

struct Setup {
    alice_client: Arc<StartedClient>,
    bob_client: Arc<StartedClient>,
    workspace_id: VlobID,
}

async fn setup_workspace(alice: Arc<LocalDevice>, bob: Arc<LocalDevice>) -> Setup {
    log::debug!("Create a workspace for alice");
    let alice_client = start_client(alice).await.unwrap();

    let wid = alice_client
        .create_workspace("new-workspace".parse().unwrap())
        .await
        .unwrap();
    log::trace!("Workspace ID: {wid}");

    alice_client.ensure_workspaces_bootstrapped().await.unwrap();
    log::debug!("Share the workspace with bob as a contributor");
    alice_client
        .share_workspace(wid, bob.user_id, Some(libparsec::RealmRole::Contributor))
        .await
        .unwrap();

    log::debug!("Ensure bob has access to the workspace");
    let bob_client = start_client(bob).await.unwrap();
    bob_client.poll_server_for_new_certificates().await.unwrap();
    bob_client.refresh_workspaces_list().await.unwrap();
    let bob_wksp_list = bob_client.list_workspaces().await;
    assert!(
        bob_wksp_list.iter().any(|wksp| wksp.id == wid),
        "Bob does not see the workspace"
    );

    log::debug!("Create a dummy file that is not synced by alice");
    let workspace = alice_client.start_workspace(wid).await.unwrap();
    workspace
        .create_file("/foo.txt".parse().unwrap())
        .await
        .unwrap();
    alice_client.stop_workspace(wid).await;

    Setup {
        alice_client,
        bob_client,
        workspace_id: wid,
    }
}

async fn find_foo_file(client: &Client, wid: VlobID) -> Option<(EntryName, EntryStat)> {
    let workspace = client.start_workspace(wid).await.unwrap();
    let entries = workspace
        .stat_folder_children(&"/".parse().unwrap())
        .await
        .unwrap();

    let expected_filename = "foo.txt".parse().unwrap();
    let res = entries
        .into_iter()
        .find(|(name, _)| name == &expected_filename);

    client.stop_workspace(wid).await;

    res
}

/// Here we test the sync command where alice (the "sender") need to sync it's workspace for bob to be able to see the changes.
/// The goal is to test that the sync command perform an outbound sync.
#[rstest::rstest]
#[tokio::test]
async fn workspace_sync_alice_need_to_sync(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let Setup {
        alice_client,
        bob_client,
        workspace_id,
    } = setup_workspace(alice.clone(), bob.clone()).await;

    // Ensure bob does not see the file foo.txt because alice is not synced
    assert!(find_foo_file(&bob_client, workspace_id).await.is_none());

    drop(alice_client); // Drop needed to release the IPC lock

    // Alice sync its changes
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "sync",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &workspace_id.hex()
    );

    // Bob also need to sync for remote change
    {
        let workspace = bob_client.start_workspace(workspace_id).await.unwrap();
        workspace.refresh_realm_checkpoint().await.unwrap();
        loop {
            let entries_to_sync = workspace.get_need_inbound_sync(32).await.unwrap();
            if entries_to_sync.is_empty() {
                break;
            }
            for entry in entries_to_sync {
                workspace.inbound_sync(entry).await.unwrap();
            }
        }
        bob_client.stop_workspace(workspace_id).await;
    }

    assert!(find_foo_file(&bob_client, workspace_id).await.is_some());
}

/// Here we test the sync command where bob (the "receiver") need to sync it's workspace to see the changes made by alice.
/// The goal is to test that the command perform an inbound sync.
#[rstest::rstest]
#[tokio::test]
async fn workspace_sync_bob_need_to_sync(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let Setup {
        alice_client,
        bob_client,
        workspace_id,
    } = setup_workspace(alice.clone(), bob.clone()).await;

    // Alice sync its changes
    {
        let workspace = alice_client.start_workspace(workspace_id).await.unwrap();
        workspace.refresh_realm_checkpoint().await.unwrap();
        loop {
            let entries_to_sync = workspace.get_need_outbound_sync(32).await.unwrap();
            if entries_to_sync.is_empty() {
                break;
            }
            for entry in entries_to_sync {
                workspace.outbound_sync(entry).await.unwrap();
            }
        }
        alice_client.stop_workspace(workspace_id).await;
    }

    // Ensure bob does not see the file foo.txt because he is not synced
    {
        assert!(find_foo_file(&bob_client, workspace_id).await.is_none());
        drop(bob_client); // Drop needed to release the IPC lock
    }

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "sync",
        "--device",
        &bob.device_id.hex(),
        "--workspace",
        &workspace_id.hex()
    );

    let bob_client = start_client(bob.clone()).await.unwrap();
    assert!(find_foo_file(&bob_client, workspace_id).await.is_some());
}
