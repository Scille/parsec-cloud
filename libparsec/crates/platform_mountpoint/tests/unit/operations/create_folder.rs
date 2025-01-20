// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client::{workspace::WorkspaceOps, Client};
use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let new_dir = mountpoint_path.join("new_dir");

            tokio::fs::create_dir(&new_dir).await.unwrap();

            let stats = wksp1_ops
                .stat_folder_children(&"/new_dir".parse().unwrap())
                .await
                .unwrap();
            assert!(stats.is_empty());
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
            let new_dir = mountpoint_path.join(target_name);

            #[cfg(not(target_os = "windows"))]
            {
                // TODO: LOOKUP_HOOK is not implemented on Windows yet
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

                let entries = tokio::fs::read_dir(mountpoint_path.clone()).await.unwrap();
                println!("Entries: {:?}", entries);

                let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
                p_assert_matches!(err.kind(), std::io::ErrorKind::AlreadyExists);
            }

            #[cfg(target_os = "windows")]
            {
                let new_dir_utf16 =
                    super::utils::to_null_terminated_utf16(&new_dir.to_string_lossy());
                let ret = unsafe {
                    windows_sys::Win32::Storage::FileSystem::CreateDirectoryW(
                        new_dir_utf16.as_ptr(),
                        std::ptr::null_mut(),
                    )
                };

                p_assert_eq!(ret, 0);
                let err = std::io::Error::last_os_error();

                p_assert_eq!(
                    err.raw_os_error(),
                    Some(windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS as i32),
                    "{}",
                    err
                );
            }
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn parent_doesnt_exist(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let new_dir = mountpoint_path.join("dummy/new_dir");

            #[cfg(not(target_os = "windows"))]
            {
                // TODO: LOOKUP_HOOK is not implemented on Windows yet
                // Prevent lookup from discovering the file doesn't exist, which would bypass
                // entirely the create_dir call
                {
                    use libparsec_client::workspace::EntryStat;
                    use libparsec_types::prelude::*;

                    let mut guard = crate::LOOKUP_HOOK.lock().unwrap();
                    *guard = Some(Box::new(move |path| {
                        if path == &"/dummy".parse().unwrap() {
                            Some(Ok(EntryStat::Folder {
                                confinement_point: None,
                                id: VlobID::default(),
                                parent: VlobID::default(),
                                created: "2000-01-01T00:00:00Z".parse().unwrap(),
                                updated: "2000-01-01T00:00:00Z".parse().unwrap(),
                                base_version: 0,
                                is_placeholder: false,
                                need_sync: false,
                            }))
                        } else {
                            // Fallback to real lookup
                            None
                        }
                    }));
                }

                let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
                p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
            }

            #[cfg(target_os = "windows")]
            {
                let new_dir_utf16 =
                    super::utils::to_null_terminated_utf16(&new_dir.to_string_lossy());
                let ret = unsafe {
                    windows_sys::Win32::Storage::FileSystem::CreateDirectoryW(
                        new_dir_utf16.as_ptr(),
                        std::ptr::null_mut(),
                    )
                };

                p_assert_eq!(ret, 0);
                let err = std::io::Error::last_os_error();

                p_assert_eq!(
                    err.raw_os_error(),
                    Some(windows_sys::Win32::Foundation::ERROR_PATH_NOT_FOUND as i32),
                    "{}",
                    err
                );
            }
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

// TODO: Fix for Windows
#[cfg(not(target_os = "windows"))]
#[parsec_test(testbed = "minimal_client_ready")]
async fn invalid_name(tmp_path: TmpPath, env: &TestbedEnv) {
    // We need to craft a name that is valid in the context of the OS, but invalid
    // for Parsec.

    #[cfg(target_os = "windows")]
    let raw_bad_name = b"new\xC0dir";

    // 0xC0 is not a valid UF8 character, however Linux rules are "no / and no \0"
    #[cfg(target_family = "unix")]
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
            let new_dir = mountpoint_path.join("foo/new_dir");
            let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
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

        let new_dir = mountpoint_path.join("foo/new_dir");
        let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
        p_assert_matches!(err.kind(), std::io::ErrorKind::NotFound);
    });
}

#[parsec_test(testbed = "coolorg")]
async fn read_only_realm(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(as "bob@dev1", env, &tmp_path, |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
        let new_dir = mountpoint_path.join("new_dir");

        let err = tokio::fs::create_dir(&new_dir).await.unwrap_err();
        #[cfg(not(target_os = "windows"))]
        p_assert_eq!(err.raw_os_error(), Some(libc::EROFS), "{}", err);
        #[cfg(target_os = "windows")]
        p_assert_eq!(err.raw_os_error(), Some(windows_sys::Win32::Foundation::ERROR_WRITE_PROTECT as i32), "{}", err);
    });
}
