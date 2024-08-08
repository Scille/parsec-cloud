use assert_cmd::Command;

use libparsec::{tmp_path, HumanHandle, TmpPath};

use super::{get_testenv_config, run_local_organization, set_env};
use crate::utils::load_client;

#[rstest::rstest]
#[tokio::test]
// This test seems to fail because alice's device ID is no longer stable (it used
// to be a string, now it's a UUID regenerated at each run), hence the test process
// and the cli invocation process have different values for `alice.device_id.hex()` !
#[ignore = "TODO: fix this test !"]
async fn share_workspace(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, _, bob], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    // FIXME: The test should not rely on the load_client_and_run since it use the stdin to read the password to unlock the device.
    let client = load_client(
        &libparsec::get_default_config_dir(),
        Some(alice.device_id.to_string()),
        false,
    )
    .await
    .unwrap();
    let wid = client
        .create_workspace("new-workspace".parse().unwrap())
        .await
        .unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();

    client.poll_server_for_new_certificates().await.unwrap();
    let users = client.list_users(false, None, None).await.unwrap();
    let bob_id = &users
        .iter()
        .find(|x| x.human_handle == HumanHandle::new("bob@example.com", "Bob").unwrap())
        .unwrap()
        .id;

    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "share-workspace",
            "--device",
            &alice.device_id.hex(),
            "--workspace-id",
            &wid.hex(),
            "--user-id",
            &bob_id.hex(),
            "--role",
            "contributor",
        ])
        .assert()
        .stdout(predicates::str::contains("Workspace has been shared"));

    // TODO: Replace me with a call to list-workspaces from a new `Arc<Client>` initialized with bob's device ID
    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args(["list-workspaces", "--device", &bob.device_id.hex()])
        .assert()
        .stdout(predicates::str::contains("new-workspace: contributor"));
}
