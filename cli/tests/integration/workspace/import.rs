use std::sync::Arc;

use libparsec::{tmp_path, EntryName, LocalDevice, TmpPath, VlobID};

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::utils::start_client;

#[rstest::rstest]
#[tokio::test]
async fn workspace_import_file(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    // Initialize workspace
    let wid = {
        let alice_client = start_client(alice.clone()).await.unwrap();

        // Create the workspace used to copy the file to
        let wid = alice_client
            .create_workspace("new-workspace".parse().unwrap())
            .await
            .unwrap();
        alice_client.ensure_workspaces_bootstrapped().await.unwrap();
        alice_client
            .share_workspace(wid, bob.user_id, Some(libparsec::RealmRole::Reader))
            .await
            .unwrap();

        alice_client.stop().await;

        wid
    };

    // Create a file to import
    let file = tmp_path.join("test.txt");
    std::fs::write(&file, "Hello, World!").unwrap();

    // Import the file
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "import",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        &file.to_string_lossy(),
        "/test.txt"
    )
    .stdout(predicates::str::is_empty());

    let bob_client = start_client(bob.clone()).await.unwrap();
    bob_client.poll_server_for_new_certificates().await.unwrap();
    bob_client.refresh_workspaces_list().await.unwrap();
    let workspace = bob_client.start_workspace(wid).await.unwrap();
    workspace.refresh_realm_checkpoint().await.unwrap();
    loop {
        let entries_to_sync = workspace.get_need_inbound_sync(32).await.unwrap();
        log::debug!("Entries to inbound sync: {entries_to_sync:?}");
        if entries_to_sync.is_empty() {
            break;
        }
        for entry in entries_to_sync {
            workspace.inbound_sync(entry).await.unwrap();
        }
    }
    let entries = workspace
        .stat_folder_children(&"/".parse().unwrap())
        .await
        .unwrap();

    assert_eq!(entries.len(), 1);
    let (name, stat) = &entries[0];
    assert_eq!(name.as_ref(), "test.txt");
    assert!(matches!(stat, libparsec::EntryStat::File { size, .. } if size == &13));
}

#[rstest::rstest]
#[tokio::test]
async fn workspace_import_file_to_specific_path(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    // Initialize workspace
    let wid = {
        let alice_client = start_client(alice.clone()).await.unwrap();

        // Create the workspace used to copy the file to
        let wid = alice_client
            .create_workspace("new-workspace".parse().unwrap())
            .await
            .unwrap();
        alice_client.ensure_workspaces_bootstrapped().await.unwrap();
        alice_client
            .share_workspace(wid, bob.user_id, Some(libparsec::RealmRole::Reader))
            .await
            .unwrap();

        alice_client.stop().await;

        wid
    };

    // Create a file to import
    let file = tmp_path.join("test.txt");
    std::fs::write(&file, "Hello, World!").unwrap();

    // Import the file to a non-existent path, should fail without `--parents` flag
    crate::assert_cmd_failure!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "import",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        &file.to_string_lossy(),
        "/a/b/c/test.txt" // path does not exists
    )
    .stderr(predicates::str::contains("Path doesn't exist"));

    // Import the file to a non-existent path, should not fail with the `--parents` flag
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "import",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        "--parents",
        &file.to_string_lossy(),
        "/d/e/f/test.txt" // path does not exists but it should be created
    )
    .stdout(predicates::str::is_empty());

    // Import the file again to the same folder
    // Using `--parents` shouldn't fail even if the path already exists
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "import",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        "--parents",
        &file.to_string_lossy(),
        "/d/e/f/test.txt" // path exists but it should not fail
    )
    .stdout(predicates::str::is_empty());

    let bob_client = start_client(bob.clone()).await.unwrap();
    bob_client.poll_server_for_new_certificates().await.unwrap();
    bob_client.refresh_workspaces_list().await.unwrap();
    let workspace = bob_client.start_workspace(wid).await.unwrap();
    workspace.refresh_realm_checkpoint().await.unwrap();
    loop {
        let entries_to_sync = workspace.get_need_inbound_sync(32).await.unwrap();
        log::debug!("Entries to inbound sync: {entries_to_sync:?}");
        if entries_to_sync.is_empty() {
            break;
        }
        for entry in entries_to_sync {
            workspace.inbound_sync(entry).await.unwrap();
        }
    }
    let entries = workspace
        .stat_folder_children(&"/d/e/f/".parse().unwrap())
        .await
        .unwrap();

    assert_eq!(entries.len(), 1);
    let (name, stat) = &entries[0];
    assert_eq!(name.as_ref(), "test.txt");
    assert!(matches!(stat, libparsec::EntryStat::File { size, .. } if size == &13));
}

#[rstest::rstest]
#[tokio::test]
async fn issue_8941_import_file_where_inbound_and_outbound_sync_are_required(tmp_path: TmpPath) {
    let (
        _,
        TestOrganization {
            alice, other_alice, ..
        },
        _,
    ) = bootstrap_cli_test(&tmp_path).await.unwrap();

    // Create a workspace C with device A
    let wid = create_workspace(alice.clone()).await;

    // Refresh the workspace list for device B
    refresh_workspace_list(other_alice).await;

    // Create a file to import
    let test_file = tmp_path.join("test.txt");
    tokio::fs::write(&test_file, "Hello, World!").await.unwrap();

    // Try to import a file with device B to workspace C
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "import",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        &test_file.to_string_lossy(),
        "/test.txt"
    )
    .stdout(predicates::str::is_empty());

    // Try to import the file again
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "import",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        &test_file.to_string_lossy(),
        "/test.txt"
    )
    .stdout(predicates::str::is_empty());
}

async fn create_workspace(device: Arc<LocalDevice>) -> VlobID {
    let client = start_client(device).await.unwrap();
    let workspace_name = "new-workspace".parse::<EntryName>().unwrap();
    let wid = client
        .create_workspace(workspace_name.clone())
        .await
        .unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();
    client.stop().await;
    wid
}

async fn refresh_workspace_list(device: Arc<LocalDevice>) {
    let client = start_client(device).await.unwrap();
    client.list_workspaces().await;
    client.stop().await;
}

#[rstest::rstest]
#[tokio::test]
async fn workspace_import_folder(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    // Initialize workspace
    let wid = {
        let alice_client = start_client(alice.clone()).await.unwrap();

        // Create the workspace used to copy the file to
        let wid = alice_client
            .create_workspace("new-workspace".parse().unwrap())
            .await
            .unwrap();
        alice_client.ensure_workspaces_bootstrapped().await.unwrap();
        alice_client
            .share_workspace(wid, bob.user_id, Some(libparsec::RealmRole::Reader))
            .await
            .unwrap();

        alice_client.stop().await;

        wid
    };

    // Create a file hierarchy
    let folder = tmp_path.join("a_folder");
    let subfolder = folder.join("subfolder");
    std::fs::create_dir_all(&subfolder).unwrap();
    let file_a = folder.join("hello.txt");
    std::fs::write(&file_a, "Hello, World!").unwrap();
    let file_b = subfolder.join("wid");
    std::fs::write(&file_b, wid.to_string()).unwrap();

    // Try to import the folder
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "import",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        &folder.to_string_lossy(),
        "/test"
    )
    .stdout(predicates::str::is_empty());

    let bob_client = start_client(bob.clone()).await.unwrap();
    bob_client.poll_server_for_new_certificates().await.unwrap();
    bob_client.refresh_workspaces_list().await.unwrap();
    let workspace = bob_client.start_workspace(wid).await.unwrap();
    workspace.refresh_realm_checkpoint().await.unwrap();
    loop {
        let entries_to_sync = workspace.get_need_inbound_sync(32).await.unwrap();
        log::debug!("Entries to inbound sync: {entries_to_sync:?}");
        if entries_to_sync.is_empty() {
            break;
        }
        for entry in entries_to_sync {
            workspace.inbound_sync(entry).await.unwrap();
        }
    }
    let mut entries = workspace
        .stat_folder_children(&"/test".parse().unwrap())
        .await
        .unwrap();

    assert_eq!(entries.len(), 2);
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let (name, stat) = entries.pop().unwrap();
    assert_eq!(name.as_ref(), "subfolder");
    assert!(matches!(stat, libparsec::EntryStat::Folder { .. }));

    let (name, stat) = entries.pop().unwrap();
    assert_eq!(name.as_ref(), "hello.txt");
    assert!(matches!(stat, libparsec::EntryStat::File { size, .. } if size == 13));

    let mut entries = workspace
        .stat_folder_children(&"/test/subfolder".parse().unwrap())
        .await
        .unwrap();
    assert_eq!(entries.len(), 1);

    let (name, stat) = entries.pop().unwrap();
    assert_eq!(name.as_ref(), "wid");
    assert!(matches!(stat, libparsec::EntryStat::File { size, .. } if dbg!(size) == 36));
}
