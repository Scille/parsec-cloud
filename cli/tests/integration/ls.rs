use libparsec::{tmp_path, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use super::bootstrap_cli_test;
use crate::testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD};
use parsec_cli::utils::start_client;

#[rstest::rstest]
#[tokio::test]
async fn ls_files(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let wid = {
        let client = start_client(alice.clone()).await.unwrap();

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

        client.stop().await;

        wid
    };

    // List the files
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "ls",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex()
    )
    .stdout(predicates::str::contains("test.txt\n").and(predicates::str::contains("foo\n")));
}
