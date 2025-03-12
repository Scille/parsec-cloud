// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{os_ls, start_client_with_mountpoint_base_dir};
use crate::Mountpoint;

#[parsec_test(testbed = "minimal_client_ready")]
async fn mount_and_unmount(
    #[values(false, true)] drop_as_umount: bool,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let mountpoint_base_dir = tmp_path.join("base");
    let client =
        start_client_with_mountpoint_base_dir(env, mountpoint_base_dir.clone(), "alice@dev1").await;
    let wksp1_ops = client.start_workspace(wksp1_id).await.unwrap();

    // 1) Mount with mountpoint base dir not existing

    let mountpoint_dir = {
        let mountpoint = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();

        let mountpoint_dir = mountpoint.path().to_owned();

        if !drop_as_umount {
            mountpoint.unmount().await.unwrap();
        }

        mountpoint_dir
    };

    // Mountpoint base dir should exist but be empty
    p_assert_matches!(
        tokio::fs::read_dir(&mountpoint_base_dir)
            .await
            .unwrap()
            .next_entry()
            .await
            .unwrap(),
        None
    );

    // 2) Mount with mountpoint base dir existing, but mountpoint dir not existing

    {
        let mountpoint = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();

        let new_mountpoint_dir = mountpoint.path().to_owned();
        p_assert_eq!(new_mountpoint_dir, mountpoint_dir);

        if !drop_as_umount {
            mountpoint.unmount().await.unwrap();
        }
    }

    // Mountpoint base dir should exist but be empty
    p_assert_matches!(
        tokio::fs::read_dir(&mountpoint_base_dir)
            .await
            .unwrap()
            .next_entry()
            .await
            .unwrap(),
        None
    );

    // 3) Mount with mountpoint base dir and mountpoint dir both existing

    tokio::fs::create_dir(&mountpoint_dir).await.unwrap();

    let actual_mountpoint_dir = {
        let mountpoint = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();
        let actual_mountpoint_dir = mountpoint.path().to_owned();

        if !drop_as_umount {
            mountpoint.unmount().await.unwrap();
        }

        actual_mountpoint_dir
    };

    // On Windows, mount has been done in a new directory, hence mountpoint dir has been untouched
    #[cfg(target_os = "windows")]
    {
        p_assert_eq!(actual_mountpoint_dir, mountpoint_base_dir.join("wksp1 (2)"));
        // Mountpoint base dir should exist with it empty child mountpoint dir
        p_assert_matches!(
            tokio::fs::read_dir(&mountpoint_dir)
                .await
                .unwrap()
                .next_entry()
                .await
                .unwrap(),
            None,
        );

        // Cleanup for next test
        tokio::fs::remove_dir(&mountpoint_dir).await.unwrap();
    }

    // On Unix, mountpoint dir has been used for the mountpoint, and hence it has been
    // removed by the unmount operation.
    #[cfg(not(target_os = "windows"))]
    {
        p_assert_eq!(actual_mountpoint_dir, mountpoint_dir);
        // Mountpoint base dir should exist but be empty
        p_assert_matches!(
            tokio::fs::read_dir(&mountpoint_base_dir)
                .await
                .unwrap()
                .next_entry()
                .await
                .unwrap(),
            None
        );
    }

    // 4) Mountpoint dir exist, but cannot be re-used (not a directory)

    tokio::fs::write(&mountpoint_dir, b"hello").await.unwrap();

    let new_mountpoint_dir = {
        let mountpoint = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();

        let new_mountpoint_dir = mountpoint.path().to_owned();
        p_assert_eq!(new_mountpoint_dir, mountpoint_base_dir.join("wksp1 (2)"));

        if !drop_as_umount {
            mountpoint.unmount().await.unwrap();
        }

        new_mountpoint_dir
    };

    // Bad mountpoint dir shouldn't have been removed
    p_assert_eq!(tokio::fs::read(&mountpoint_dir).await.unwrap(), b"hello");
    // New mountpoint dir should have been removed
    p_assert_eq!(
        tokio::fs::metadata(&new_mountpoint_dir)
            .await
            .unwrap_err()
            .kind(),
        std::io::ErrorKind::NotFound
    );

    // Clean up for next test
    tokio::fs::remove_file(&mountpoint_dir).await.unwrap();

    // 5) Mountpoint dir exist, but cannot be re-used (directory not empty)

    tokio::fs::create_dir(&mountpoint_dir).await.unwrap();
    tokio::fs::write(&mountpoint_dir.join("child.txt"), b"hello")
        .await
        .unwrap();

    {
        let mountpoint = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();

        p_assert_eq!(mountpoint.path(), mountpoint_base_dir.join("wksp1 (2)"));

        if !drop_as_umount {
            mountpoint.unmount().await.unwrap();
        }
    }

    // Our data should still be there
    p_assert_eq!(
        tokio::fs::read(&mountpoint_dir.join("child.txt"))
            .await
            .unwrap(),
        b"hello"
    );

    // Clean up for next test
    tokio::fs::remove_dir_all(&mountpoint_dir).await.unwrap();

    // 6) Mountpoint dir exist, but cannot be re-used (already mounted)

    {
        let mountpoint1 = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();
        let mountpoint2 = Mountpoint::mount(wksp1_ops.clone()).await.unwrap();

        let new_mountpoint_dir = mountpoint2.path().to_owned();
        p_assert_eq!(new_mountpoint_dir, mountpoint_base_dir.join("wksp1 (2)"));

        if !drop_as_umount {
            mountpoint2.unmount().await.unwrap();
            mountpoint1.unmount().await.unwrap();
        }
    };

    // Mountpoint base dir should exist but be empty
    p_assert_matches!(
        tokio::fs::read_dir(&mountpoint_base_dir)
            .await
            .unwrap()
            .next_entry()
            .await
            .unwrap(),
        None
    );

    client.stop().await;
}

#[parsec_test(testbed = "minimal_client_ready")]
// UTF8 name
#[case("国家", "国家")]
// Character not allowed on Windows
#[case("\\", "~5c")]
#[case(":", "~3a")]
// cspell:disable-next-line
#[case("foo:bar", "foo~3abar")]
// Backslash shouldn't lead to multiple parts in the path
// cspell:disable-next-line
#[case("foo\\bar", "foo~5cbar")]
// UNC absolute path
// cspell:disable-next-line
#[case("\\\\hello\\", "~5c~5chello~5c")]
// cspell:disable-next-line
#[case("\\\\hello", "~5c~5chello")]
// cspell:disable-next-line
#[case("\\\\127.0.0.1\\foo", "~5c~5c127.0.0.1~5cfoo")]
#[case("\\\\127.0.0.1", "~5c~5c127.0.0.1")]
// File name not allowed on Windows
#[case("AUX", "AU~58")]
#[case("COM8.txt", "COM~38.txt")]
async fn weird_workspace_name(
    #[case] workspace_name: &str,
    #[cfg_attr(not(windows), allow(unused_variables))]
    #[case]
    windows_expected_mountpoint_name: &str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        builder.rename_realm(wksp1_id, workspace_name);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .user_storage_local_update("alice@dev1")
            .update_local_workspaces_with_fetched_certificates();
    })
    .await;

    let mountpoint_base_dir = tmp_path.join("base");
    let client =
        start_client_with_mountpoint_base_dir(env, mountpoint_base_dir.clone(), "alice@dev1").await;
    let wksp1_ops = client.start_workspace(wksp1_id).await.unwrap();
    p_assert_eq!(
        wksp1_ops.get_current_name_and_self_role().0.as_ref(),
        workspace_name
    ); // Sanity check

    let mountpoint = Mountpoint::mount(wksp1_ops).await.unwrap();

    #[cfg(not(windows))]
    let expected_mountpoint_name: &str = workspace_name;
    #[cfg(windows)]
    let expected_mountpoint_name: &str = windows_expected_mountpoint_name;

    p_assert_eq!(
        mountpoint.path().parent(),
        Some(mountpoint_base_dir.as_ref())
    );
    p_assert_eq!(
        mountpoint.path().file_name(),
        Some(std::ffi::OsStr::new(expected_mountpoint_name))
    );
    p_assert_eq!(
        os_ls!(mountpoint_base_dir).await,
        [expected_mountpoint_name]
    );
}
