// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{
    workspace::{EntryStat, WorkspaceOps},
    Client,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_first_open(
    #[values(
        "existing",
        "truncate_existing",
        "create_on_existing",
        "create_on_non_existing",
        "create_new_on_non_existing"
    )]
    kind: &str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut open_options = tokio::fs::OpenOptions::new();
            let (name, expected_size) = match kind {
                "existing" => {
                    open_options.read(true);
                    ("bar.txt", 11)
                }
                "truncate_existing" => {
                    open_options.write(true);
                    open_options.truncate(true);
                    ("bar.txt", 0)
                }
                "create_on_existing" => {
                    open_options.write(true);
                    open_options.create(true);
                    ("bar.txt", 11)
                }
                "create_on_non_existing" => {
                    open_options.write(true);
                    open_options.create(true);
                    ("new.txt", 0)
                }
                "create_new_on_non_existing" => {
                    open_options.write(true);
                    open_options.create_new(true);
                    ("new.txt", 0)
                }
                unknown => panic!("Unknown kind: {}", unknown),
            };

            // Scope to ensure when the drop occurs, as it closes the file
            {
                open_options
                    .open(&mountpoint_path.join(name))
                    .await
                    .unwrap();
            }

            let stat = wksp1_ops
                .stat_entry(&format!("/{}", name).parse().unwrap())
                .await
                .unwrap();
            p_assert_matches!(stat, EntryStat::File { size, .. } if size == expected_size);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_already_opened(
    #[values("existing", "truncate_existing", "create_on_existing")] kind: &str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut open_options = tokio::fs::OpenOptions::new();
            let (name, expected_size) = match kind {
                "existing" => {
                    open_options.read(true);
                    ("bar.txt", 11)
                }
                "truncate_existing" => {
                    open_options.write(true);
                    open_options.truncate(true);
                    ("bar.txt", 0)
                }
                "create_on_existing" => {
                    open_options.write(true);
                    open_options.create(true);
                    ("bar.txt", 11)
                }
                unknown => panic!("Unknown kind: {}", unknown),
            };

            let _fd0 = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&mountpoint_path.join(name))
                .await
                .unwrap();

            open_options
                .open(&mountpoint_path.join(name))
                .await
                .unwrap();

            let stat = wksp1_ops
                .stat_entry(&format!("/{}", name).parse().unwrap())
                .await
                .unwrap();
            p_assert_matches!(stat, EntryStat::File { size, .. } if size == expected_size);
        }
    );
}

// TODO: Fix for Windows
#[cfg(not(target_os = "windows"))]
#[parsec_test(testbed = "minimal_client_ready")]
async fn no_create_and_not_found(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Prevent lookup from discovering the file doesn't exist, which would bypass
            // the open call
            {
                let mut guard = crate::LOOKUP_HOOK.lock().unwrap();
                *guard = Some(Box::new(move |path| {
                    if path == &"/dummy.txt".parse().unwrap() {
                        Some(Ok(EntryStat::File {
                            confinement_point: None,
                            id: VlobID::default(),
                            created: "2000-01-01T00:00:00Z".parse().unwrap(),
                            updated: "2000-01-01T00:00:00Z".parse().unwrap(),
                            base_version: 0,
                            is_placeholder: false,
                            need_sync: false,
                            size: 0,
                        }))
                    } else {
                        // Fallback to real lookup
                        None
                    }
                }));
            }

            let err = tokio::fs::read(&mountpoint_path.join("dummy.txt"))
                .await
                .unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
        }
    );
}

// TODO: Fix for Windows
#[cfg(not(target_os = "windows"))]
#[parsec_test(testbed = "minimal_client_ready")]
async fn create_new_and_already_exists(tmp_path: TmpPath, env: &TestbedEnv) {
    use libparsec_client::workspace::WorkspaceStatEntryError;

    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Prevent lookup from discovering the file exists, which would bypass
            // the open call
            {
                let mut guard = crate::LOOKUP_HOOK.lock().unwrap();
                *guard = Some(Box::new(move |path| {
                    if path == &"/bar.txt".parse().unwrap() {
                        Some(Err(WorkspaceStatEntryError::EntryNotFound))
                    } else {
                        // Fallback to real lookup
                        None
                    }
                }));
            }

            let err = tokio::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap_err();
            p_assert_matches!(err.kind(), std::io::ErrorKind::AlreadyExists);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(
    #[values("open_only", "create_open")] kind: &str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    mount_and_test!(
        env,
        &tmp_path,
        |client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            client.stop_workspace(wksp1_ops.realm_id()).await;

            let mut open_options = tokio::fs::OpenOptions::new();
            match kind {
                "open_only" => {
                    open_options.read(true);
                }
                "create_open" => {
                    open_options.write(true).create(true);
                }
                unknown => panic!("Unknown kind: {}", unknown),
            }

            let err = open_options
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap_err();
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
async fn offline(
    #[values("open_only", "create_open")] kind: &str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
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
            let mut open_options = tokio::fs::OpenOptions::new();
            match kind {
                "open_only" => {
                    open_options.read(true);
                }
                "create_open" => {
                    open_options.write(true).create(true);
                }
                unknown => panic!("Unknown kind: {}", unknown),
            }
            let err = open_options
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap_err();
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
async fn read_only_realm(
    #[values(
        "open_for_read",
        "open_for_write",
        "open_for_read_write",
        "create_open"
    )]
    kind: &str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    // Add a `bar.txt` file to wksp1
    env.customize(|builder| {
        let wksp1_id: VlobID = *builder.get_stuff("wksp1_id");

        let bar_txt_id = builder
            .create_or_update_file_manifest_vlob("alice@dev1", wksp1_id, None)
            .map(|e| e.manifest.id);

        builder
            .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
            .customize(|e| {
                let manifest = Arc::make_mut(&mut e.manifest);
                manifest
                    .children
                    .insert("bar.txt".parse().unwrap(), bar_txt_id);
            });

        builder.workspace_data_storage_fetch_workspace_vlob("bob@dev1", wksp1_id, None);
        builder.workspace_data_storage_fetch_file_vlob("bob@dev1", wksp1_id, bar_txt_id);
    });

    mount_and_test!(as "bob@dev1", &env, &tmp_path, |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
        let mut open_options = tokio::fs::OpenOptions::new();
        let (should_succeed, name) = match kind {
            "open_for_read" => { open_options.read(true); (true, "bar.txt") },
            "open_for_write" => { open_options.write(true); (false, "bar.txt") },
            "open_for_read_write" => { open_options.read(true).write(true); (false, "bar.txt") },
            "create_open" => { open_options.write(true).create(true); (false, "new.txt") },
            unknown => panic!("Unknown kind: {}", unknown),
        };

        let path = mountpoint_path.join(name);
        if should_succeed {
            open_options.open(&path).await.unwrap();
        } else {
            let err = open_options.open(&path).await.unwrap_err();
            // TODO: change error to be closer to
            #[cfg(not(target_os = "windows"))]
            p_assert_matches!(err.kind(), std::io::ErrorKind::PermissionDenied);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_WRITE_PROTECT as i32),
                "{}",
                err
            );
        }
    });
}
