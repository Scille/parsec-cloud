// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{
    workspace::{EntryStat, WorkspaceOps},
    Client,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{mount_and_test, start_client};
use crate::Mountpoint;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let new_dir = mountpoint_path.join("new_dir");

            tokio::fs::create_dir(&new_dir).await.unwrap();

            let stat = wksp1_ops
                .stat_entry(&"/new_dir".parse().unwrap())
                .await
                .unwrap();
            p_assert_matches!(stat, EntryStat::Folder{children, ..} if children.is_empty());
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn already_exists(
    #[values("exist_as_file", "exist_as_folder")] kind: &'static str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let target_name = match kind {
                "exist_as_file" => "bar.txt",
                "exist_as_folder" => "foo",
                unknown => panic!("Unknown kind: {}", unknown),
            };

            // Prevent lookup from discovering the file exists, which would bypass
            // entirely the create_dir call
            {
                let mut guard = crate::LOOKUP_HOOK.lock().unwrap();
                *guard = Some(Box::new(move |path| {
                    if path == &format!("/{}", target_name).parse().unwrap() {
                        Some(Err(
                            libparsec_client::workspace::WorkspaceStatEntryError::EntryNotFound,
                        ))
                    } else {
                        // Fallback to real lookup
                        None
                    }
                }));
            }

            let new_dir = mountpoint_path.join(target_name);

            let entries = tokio::fs::read_dir(mountpoint_path.clone()).await.unwrap();
            println!("Entries: {:?}", entries);

            let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::AlreadyExists);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn parent_doesnt_exist(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Prevent lookup from discovering the file doesn't exist, which would bypass
            // entirely the create_dir call
            {
                let mut guard = crate::LOOKUP_HOOK.lock().unwrap();
                *guard = Some(Box::new(move |path| {
                    if path == &"/dummy".parse().unwrap() {
                        Some(Ok(EntryStat::Folder {
                            confinement_point: None,
                            id: VlobID::default(),
                            created: "2000-01-01T00:00:00Z".parse().unwrap(),
                            updated: "2000-01-01T00:00:00Z".parse().unwrap(),
                            base_version: 0,
                            is_placeholder: false,
                            need_sync: false,
                            children: vec![],
                        }))
                    } else {
                        // Fallback to real lookup
                        None
                    }
                }));
            }

            let new_dir = mountpoint_path.join("dummy/new_dir");

            let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn parent_is_file(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let new_dir = mountpoint_path.join("bar.txt/new_dir");

            let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
            // Cannot use `std::io::ErrorKind::NotADirectory` as it is unstable
            p_assert_eq!(err.raw_os_error(), Some(libc::ENOTDIR), "{}", err);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_name(tmp_path: TmpPath, env: &TestbedEnv) {
    // We need to craft a name that is valid in the context of the OS, but invalid
    // for Parsec.

    #[cfg(target_os = "windows")]
    let raw_bad_name = b"new\xC0dir";

    // 0xC0 is not a valid UF8 character, however Linux rules are "no / and no \0"
    #[cfg(target_os = "linux")]
    let raw_bad_name = b"new\xC0dir";

    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Prevent lookup from discovering the file exists, which would bypass
            // entirely the create_dir call
            {
                let mut guard = crate::LOOKUP_HOOK.lock().unwrap();
                *guard = Some(Box::new(move |path| {
                    if !path.is_root() {
                        Some(Err(
                            libparsec_client::workspace::WorkspaceStatEntryError::EntryNotFound,
                        ))
                    } else {
                        None
                    }
                }));
            }

            // TODO: Windows and Unix have different rules for file names
            // SAFETY: We must bypass the safety checks to craft a weird name
            let bad_name = unsafe { std::ffi::OsStr::from_encoded_bytes_unchecked(raw_bad_name) };
            let new_dir = mountpoint_path.join(bad_name);

            let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::InvalidInput);
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

            let new_dir = mountpoint_path.join("new_dir");
            let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
            p_assert_eq!(err.raw_os_error(), Some(libc::EIO), "{}", err);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(tmp_path: TmpPath, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        // Ignore all events related to workspace local storage except for the
        // workspace manifest. This way we have a root containing entries, but
        // accessing them require to fetch data from the server.
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
                    | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
                    | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageLocalWorkspaceManifestUpdate(_)
                    | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
                    | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_)
                    | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
    });
    mount_and_test!(
        &env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let new_dir = mountpoint_path.join("foo/new_dir");
            let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            p_assert_eq!(err.raw_os_error(), Some(libc::EHOSTUNREACH), "{}", err);
        }
    );
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn no_realm_access(tmp_path: TmpPath, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        // Ignore all events related to workspace local storage except for the
        // workspace manifest. This way we have a root containing entries, but
        // accessing them require to fetch data from the server.
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
                    | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
                    | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageLocalWorkspaceManifestUpdate(_)
                    | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
                    | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_)
                    | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
    });

    let alice_client = start_client(&env, "alice@dev1").await;
    mount_and_test!(as "bob@dev1", &env, &tmp_path, |bob_client: Arc<Client>, bob_wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
        // Bob lose access to the workspace while it has it mounted...

        alice_client.share_workspace(bob_wksp1_ops.realm_id(), bob_client.device_id().user_id().to_owned(), None).await.unwrap();

        // ...and only realized it when trying to communicate with the server
        // (note the monitors are not running in the client, hence no risk of
        // having a concurrent processing of the loss of access)

        let new_dir = mountpoint_path.join("foo/new_dir");
        let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
        p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
    });
}

#[parsec_test(testbed = "coolorg")]
async fn read_only_realm(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(as "bob@dev1", &env, &tmp_path, |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
        let new_dir = mountpoint_path.join("new_dir");

        let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
        p_assert_matches!(err.kind(), std::io::ErrorKind::PermissionDenied);
    });
}
