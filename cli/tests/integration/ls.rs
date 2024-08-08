use std::sync::Arc;

use assert_cmd::Command;

use libparsec::{tmp_path, ClientConfig, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use super::bootstrap_cli_test;
use crate::testenv_utils::DEFAULT_DEVICE_PASSWORD;

#[rstest::rstest]
#[tokio::test]
async fn ls_files(tmp_path: TmpPath) {
    let (_, [alice, ..], _) = bootstrap_cli_test(&tmp_path).await.unwrap();

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

    // List the files
    Command::cargo_bin("parsec_cli")
        .unwrap()
        .args([
            "ls",
            "--device",
            &alice.device_id.hex(),
            "--password-stdin",
            "--workspace-id",
            &wid.hex(),
        ])
        .write_stdin(format!("{DEFAULT_DEVICE_PASSWORD}\n"))
        .assert()
        .success()
        .stdout(predicates::str::contains("test.txt\n").and(predicates::str::contains("foo\n")));
}
