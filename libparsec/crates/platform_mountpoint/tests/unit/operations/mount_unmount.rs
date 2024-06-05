// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::start_client_with_mountpoint_base_dir;
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
