// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{
    Client,
    workspace::{WorkspaceOps, WorkspaceStatEntryError},
};
use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            p_assert_matches!(
                wksp1_ops.stat_entry(&"/foo/spam".parse().unwrap()).await,
                Ok(_)
            );

            let path = mountpoint_path.join("foo/spam");
            tokio::fs::remove_dir(&path).await.unwrap();

            p_assert_matches!(
                wksp1_ops.stat_entry(&"/foo/spam".parse().unwrap()).await,
                Err(WorkspaceStatEntryError::EntryNotFound)
            );

            // Make sur the FS also does not have the entry anymore
            let err = tokio::fs::metadata(&mountpoint_path.join("foo/spam"))
                .await
                .unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_empty(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let path = mountpoint_path.join("foo");

            let err = tokio::fs::remove_dir(&path).await.unwrap_err();
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::ENOTEMPTY), "{}", err);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_DIR_NOT_EMPTY as i32),
                "{}",
                err
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn is_file(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let path = mountpoint_path.join("bar.txt");

            let err = tokio::fs::remove_dir(&path).await.unwrap_err();
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::ENOTDIR), "{}", err);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_DIRECTORY as i32),
                "{}",
                err
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_found(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let path = mountpoint_path.join("dummy");

            let err = tokio::fs::remove_dir(&path).await.unwrap_err();
            p_assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            client.stop_workspace(wksp1_ops.realm_id()).await;

            let path = mountpoint_path.join("foo/spam");
            let err = tokio::fs::remove_dir(&path).await.unwrap_err();
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EIO), "{}", err);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_NOT_READY as i32),
                "{}",
                err
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(tmp_path: TmpPath, env: &TestbedEnv) {
    env.customize(|builder| {
        // Ignore all events related to workspace local storage except for the
        // workspace manifest. This way we have a root containing entries, but
        // accessing them require to fetch data from the server.
        builder.filter_client_storage_events(|e| match e {
            TestbedEvent::WorkspaceDataStorageFetchFolderVlob(e)
                if e.local_manifest.base.is_root() =>
            {
                true
            }
            TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
            | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
            | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_)
            | TestbedEvent::WorkspaceDataStorageChunkCreate(_) => false,
            _ => true,
        });
    })
    .await;
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let path = mountpoint_path.join("foo/spam");
            let err = tokio::fs::remove_dir(&path).await.unwrap_err();
            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EHOSTUNREACH), "{}", err);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_HOST_UNREACHABLE as i32),
                "{}",
                err
            );
        }
    );
}

#[parsec_test(testbed = "coolorg")]
async fn no_realm_access(tmp_path: TmpPath, env: &TestbedEnv) {
    env.customize(|builder| {
        // Ignore all events related to workspace local storage except for the
        // workspace manifest. This way we have a root containing entries, but
        // accessing them require to fetch data from the server.
        builder.filter_client_storage_events(|e| match e {
            TestbedEvent::WorkspaceDataStorageFetchFolderVlob(e)
                if e.local_manifest.base.is_root() =>
            {
                true
            }
            TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
            | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
            | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_)
            | TestbedEvent::WorkspaceDataStorageChunkCreate(_) => false,
            _ => true,
        });
    })
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            move |_: authenticated_cmds::latest::realm_unshare::Req| {
                authenticated_cmds::latest::realm_unshare::Rep::Ok
            }
        },
        {
            move |_: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                    realm_certificates: Default::default(),
                }
            }
        }
    );

    let alice_client = super::utils::start_client(env, "alice@dev1").await;
    mount_and_test!(as "bob@dev1", env, &tmp_path, |bob_client: Arc<Client>, bob_wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
        // Bob lose access to the workspace while it has it mounted...

        alice_client.share_workspace(bob_wksp1_ops.realm_id(), bob_client.user_id(), None).await.unwrap();

        // ...and only realized it when trying to communicate with the server
        // (note the monitors are not running in the client, hence no risk of
        // having a concurrent processing of the loss of access)

        let path = mountpoint_path.join("foo/spam");
        let err = tokio::fs::remove_dir(&path).await.unwrap_err();
        p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
    });
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn read_only_realm(tmp_path: TmpPath, env: &TestbedEnv) {
    env.customize(|builder| {
        // Share workspace with a new user Bob so that Alice is able to become a reader...
        let wksp1_id: VlobID = *builder.get_stuff("wksp1_id");
        builder.new_user("bob");
        builder.share_realm(wksp1_id, "bob", Some(RealmRole::Owner));
        builder.share_realm(wksp1_id, "alice", Some(RealmRole::Reader));
        // ...on top of that, Alice's client must be aware of the change
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .user_storage_local_update("alice@dev1")
            .update_local_workspaces_with_fetched_certificates();
    })
    .await;
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let path = mountpoint_path.join("foo/spam");
            let err = tokio::fs::remove_dir(&path).await.unwrap_err();
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EROFS), "{}", err);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_WRITE_PROTECT as i32),
                "{}",
                err
            );
        }
    );
}

// TODO: test not found (but this is tricky given we have to trick `lookup` first)
