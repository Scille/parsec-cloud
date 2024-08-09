use std::sync::Arc;

use assert_cmd::Command;

use libparsec::{tmp_path, ClientConfig, TmpPath};

use super::{get_testenv_config, run_local_organization, set_env};
use crate::testenv_utils::DEFAULT_DEVICE_PASSWORD;

#[rstest::rstest]
#[tokio::test]
async fn rm_files(tmp_path: TmpPath) {
    let _ = env_logger::builder().is_test(true).try_init();
    let tmp_path_str = tmp_path.to_str().unwrap();
    let config = get_testenv_config();
    let (url, [alice, ..], _) = run_local_organization(&tmp_path, None, config)
        .await
        .unwrap();

    set_env(tmp_path_str, &url);

    let client = libparsec::internal::Client::start(
        Arc::new(
            ClientConfig {
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

    // Create the workspace used to copy the file to
    let wid = client
        .create_workspace("new-workspace".parse().unwrap())
        .await
        .unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();

    let workspace = client.start_workspace(wid).await.unwrap();
    workspace
        .create_file("/test.txt".parse().unwrap())
        .await
        .unwrap();
    workspace
        .create_folder("/foo".parse().unwrap())
        .await
        .unwrap();
    client.stop_workspace(wid).await;
    drop(workspace);

    // Remove test.txt
    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "rm",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
            "--workspace-id",
            &wid.hex(),
            "/test.txt",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .success()
        .stdout(predicates::str::is_empty());

    // Remove foo
    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "rm",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
            "--workspace-id",
            &wid.hex(),
            "/foo",
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .success()
        .stdout(predicates::str::is_empty());

    let workspace = client.start_workspace(wid).await.unwrap();
    let entries = workspace.stat_folder_children_by_id(wid).await.unwrap();
    assert_eq!(entries.len(), 0);
}
