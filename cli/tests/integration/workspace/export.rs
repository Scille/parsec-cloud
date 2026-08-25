use libparsec::{tmp_path, FsPath, OpenOptions, TmpPath};
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    bootstrap_cli_test, test_ui,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};
use parsec_cli::{ui::Ui, utils::start_client};

#[rstest::rstest]
#[tokio::test]
async fn workspace_export_file(
    tmp_path: TmpPath,
    test_ui: &Ui,
    #[values("all", "none-fail")] update: &str,
) {
    let (_, TestOrganization { alice, .. }, _) =
        bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    let remote_path = "/hello.txt";
    let fs_path = FsPath::from_str(remote_path).unwrap();

    let local_path = tmp_path.join("hello.txt");
    let content = b"Hello, world!";
    let old_content = b"Old old stuff";

    // Create previous local file
    {
        let mut previous_file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&local_path)
            .await
            .unwrap();
        previous_file.write_all(old_content).await.unwrap();
    }

    // Initialize workspace
    let wid = {
        let alice_client = start_client(alice.clone()).await.unwrap();

        // create workspace
        let wid = alice_client
            .create_workspace("new-workspace".parse().unwrap())
            .await
            .unwrap();
        alice_client.ensure_workspaces_bootstrapped().await.unwrap();

        // create file to export
        let workspace = alice_client.start_workspace(wid).await.unwrap();
        let _ = workspace.create_file(fs_path.clone()).await.unwrap();
        let fd = workspace
            .open_file(fs_path, OpenOptions::read_write())
            .await
            .unwrap();
        workspace.fd_write(fd, 0, content).await.unwrap();

        alice_client.stop().await;

        wid
    };

    // Export the file
    macro_rules! assert_export_cmd {
        () => {
            crate::assert_cmd!(
                with_password = DEFAULT_DEVICE_PASSWORD,
                "workspace",
                "export",
                "--device",
                &alice.device_id.hex(),
                "--workspace",
                &wid.hex(),
                &remote_path,
                &local_path,
                "--update",
                &update
            )
        };
    }
    match update {
        "none-fail" => {
            assert_export_cmd!()
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error: File already exists."));
        }
        "all" => {
            assert_export_cmd!()
                .assert()
                .success()
                .stdout(predicates::str::is_empty());
        }
        _ => unimplemented!(),
    }

    let expected_content = match update {
        "all" => content,
        "none-fail" => old_content,
        _ => unimplemented!(),
    };

    let mut fd = tokio::fs::OpenOptions::new()
        .read(true)
        .open(&local_path)
        .await
        .unwrap();
    let mut buf = Vec::with_capacity(expected_content.len());
    let out = fd.read_to_end(&mut buf).await.unwrap();
    assert_ne!(out, 0);
    assert_eq!(buf, expected_content)
}

#[rstest::rstest]
#[tokio::test]
async fn workspace_export_file_parents(
    tmp_path: TmpPath,
    test_ui: &Ui,
    #[values("all", "none-fail")] update: &str,
) {
    let (_, TestOrganization { alice, .. }, _) =
        bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    let remote_path = "/hello.txt";
    let fs_path = FsPath::from_str(remote_path).unwrap();

    let local_path = tmp_path.join("not_existing_dir").join("hello.txt");
    let content = b"Hello, world!";

    // Initialize workspace
    let wid = {
        let alice_client = start_client(alice.clone()).await.unwrap();

        // create workspace
        let wid = alice_client
            .create_workspace("new-workspace".parse().unwrap())
            .await
            .unwrap();
        alice_client.ensure_workspaces_bootstrapped().await.unwrap();

        // create file to export
        let workspace = alice_client.start_workspace(wid).await.unwrap();
        let _ = workspace.create_file(fs_path.clone()).await.unwrap();
        let fd = workspace
            .open_file(fs_path, OpenOptions::read_write())
            .await
            .unwrap();
        workspace.fd_write(fd, 0, content).await.unwrap();

        alice_client.stop().await;

        wid
    };

    // Parent not existing
    crate::assert_cmd_failure!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "export",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        &remote_path,
        &local_path,
        "--update",
        &update
    )
    .stderr(predicates::str::contains(
        "Error: No such file or directory",
    ));

    // --parents to create parents
    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "workspace",
        "export",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &wid.hex(),
        &remote_path,
        &local_path,
        "--update",
        &update,
        "--parents"
    )
    .stdout(predicates::str::is_empty());

    // Export the file
    macro_rules! assert_export_cmd {
        () => {
            crate::assert_cmd!(
                with_password = DEFAULT_DEVICE_PASSWORD,
                "workspace",
                "export",
                "--device",
                &alice.device_id.hex(),
                "--workspace",
                &wid.hex(),
                &remote_path,
                &local_path,
                "--update",
                &update
            )
        };
    }

    // Parent exists so depends on update mode
    match update {
        "none-fail" => {
            assert_export_cmd!()
                .assert()
                .failure()
                .stderr(predicates::str::contains("Error: File already exists."));
        }
        "all" => {
            assert_export_cmd!()
                .assert()
                .success()
                .stdout(predicates::str::is_empty());
        }
        _ => unimplemented!(),
    }
}
