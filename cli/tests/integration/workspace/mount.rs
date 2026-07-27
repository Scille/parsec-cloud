use std::{os::linux::fs::MetadataExt, sync::Arc};

use libparsec::{LocalDevice, OpenOptions, VlobID};
use libparsec_tests_fixtures::{tmp_path, TmpPath};
use parsec_cli::{
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
    utils::start_client,
};

use crate::bootstrap_cli_test;

pub async fn setup_workspace(alice: Arc<LocalDevice>) -> VlobID {
    log::debug!("Create a workspace for alice");
    let alice_client = start_client(alice).await.unwrap();

    let wid = alice_client
        .create_workspace("new-workspace".parse().unwrap())
        .await
        .unwrap();
    log::trace!("Workspace ID: {wid}");

    alice_client.ensure_workspaces_bootstrapped().await.unwrap();

    log::debug!("Create dummy files");
    let workspace = alice_client.start_workspace(wid).await.unwrap();

    workspace
        .create_folder("/foo".parse().unwrap())
        .await
        .unwrap();
    let fd = workspace
        .open_file(
            "/bar.txt".parse().unwrap(),
            OpenOptions {
                create_new: true,
                ..OpenOptions::read_write()
            },
        )
        .await
        .unwrap();
    workspace.fd_write(fd, 0, b"hello world").await.unwrap();
    workspace.fd_close(fd).await.unwrap();

    alice_client.stop_workspace(wid).await;

    alice_client.stop().await;

    wid
}

#[rstest::rstest]
#[tokio::test]
async fn mount_workspace(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();
    let mount_dir = tmp_path.join("mountpoint");

    std::assert_matches!(
        tokio::fs::try_exists(&mount_dir).await,
        Ok(false),
        "Mountpoint dir should not exist beforehand"
    );

    tokio::fs::create_dir_all(&mount_dir).await.unwrap();

    let parent_st_dev = tokio::fs::metadata(&*tmp_path).await.unwrap().st_dev();

    let workspace_id = setup_workspace(alice.clone()).await;

    let cmd = crate::std_cmd!(
        "workspace",
        "mount",
        "--password-stdin",
        "--device",
        &alice.device_id.hex(),
        "--workspace",
        &workspace_id.hex(),
        &mount_dir.to_string_lossy()
    );
    let mut p = crate::spawn_interactive_command(cmd, Some(1500)).unwrap();

    p.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();
    p.exp_string("Workspace mount").unwrap();

    std::assert_matches!(
        tokio::fs::try_exists(&mount_dir).await,
        Ok(true),
        "Mountpoint should now exist"
    );

    let mount_st_dev = tokio::fs::metadata(&mount_dir).await.unwrap().st_dev();
    assert_ne!(
        parent_st_dev, mount_st_dev,
        "st_dev should be different since it's not the same filesystems"
    );

    let blob = tokio::fs::read_to_string(mount_dir.join("bar.txt"))
        .await
        .unwrap();
    assert_eq!(blob, "hello world");

    p.process_mut()
        .signal(rexpect::process::Signal::SIGINT)
        .unwrap();

    let status = p.process().wait().unwrap();
    std::assert_matches!(status, rexpect::process::WaitStatus::Exited(_, 0));
}
