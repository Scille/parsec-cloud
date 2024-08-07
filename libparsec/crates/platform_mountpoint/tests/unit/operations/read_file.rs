// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    io::{Read, Seek},
    path::PathBuf,
    sync::Arc,
};

use libparsec_client::{workspace::WorkspaceOps, Client};
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
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();

                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap();
                p_assert_eq!(buff, b"hello world");

                // Read with cursor exhausted
                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap();
                p_assert_eq!(buff, b"");

                // Re-wind the cursor, and read a part of the file
                fd.seek(std::io::SeekFrom::Start(3)).unwrap();
                let mut buff = vec![0; 6];
                fd.read_exact(&mut buff).unwrap();
                p_assert_eq!(buff, b"lo wor");
            })
            .await
            .unwrap();
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn read_too_much(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let err = tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();

                let mut buff = vec![0; 20];
                fd.read_exact(&mut buff).unwrap_err()
            })
            .await
            .unwrap();

            p_assert_matches!(err.kind(), std::io::ErrorKind::UnexpectedEof);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_in_read_mode(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let err = tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .write(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();

                let mut buff = vec![0; 20];
                fd.read_exact(&mut buff).unwrap_err()
            })
            .await
            .unwrap();

            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EBADF), "{}", err);
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED as i32),
                "{}",
                err
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let fd = tokio::fs::OpenOptions::new()
                .read(true)
                .open(mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            client.stop_workspace(wksp1_ops.realm_id()).await;

            // Do file read + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let mut fd = fd.into_std().await;
            let err = tokio::task::spawn_blocking(move || {
                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap_err()
                // note `fd` getts closed here
            })
            .await
            .unwrap();

            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EIO), "{}", err);
            // TODO: error is expected to be `ERROR_NOT_READY`, but due to lookup
            //       before actually doing the flush, the error is different
            #[cfg(target_os = "windows")]
            p_assert_eq!(
                err.raw_os_error(),
                Some(windows_sys::Win32::Foundation::ERROR_INVALID_HANDLE as i32),
                "{}",
                err
            );
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(tmp_path: TmpPath, env: &TestbedEnv) {
    env.customize(|builder| {
        // No chunk&block in the local storage, so server access must occur if we
        // read the content of a file !
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
    })
    .await;
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let err = tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();

                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap_err()
            })
            .await
            .unwrap();

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

// TODO: Investigate why two `block_read` are send to server
// TODO: Ignored test given `NoRealmAccess` is not currently implemented in libparsec_client
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn no_realm_access(tmp_path: TmpPath, env: &TestbedEnv) {
    env.customize(|builder| {
        let wksp1_id: VlobID = *builder.get_stuff("wksp1_id");
        // No chunk&block in the local storage, so server access must occur if we
        // read the content of a file !
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
        // Add Bob that will be used to remove realm access to Alice
        builder.new_user("bob");
        builder.share_realm(wksp1_id, "bob", Some(RealmRole::Owner));
        builder.share_realm(wksp1_id, "alice", None);
        // So Alice has lost the access to the workspace, but doesn't know it yet...
    })
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::block_read::Req| {
            authenticated_cmds::latest::block_read::Rep::AuthorNotAllowed
        },
        move |_req: authenticated_cmds::latest::block_read::Req| {
            authenticated_cmds::latest::block_read::Rep::AuthorNotAllowed
        },
    );

    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let err = tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();

                // ...and only realized it when she tries to communicate with the server
                // (note the monitors are not running in the client, hence no risk of
                // having a concurrent processing of the loss of access)

                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap_err()
            })
            .await
            .unwrap();

            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EACCES), "{}", err);
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

// TODO: Ignored test given `StoreUnavailable` is not currently implemented in libparsec_client
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn server_block_read_but_store_unavailable(tmp_path: TmpPath, env: &TestbedEnv) {
    env.customize(|builder| {
        // No chunk&block in the local storage, so server access must occur if we
        // read the content of a file !
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
    })
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::block_read::Req| {
            authenticated_cmds::latest::block_read::Rep::StoreUnavailable
        },
    );

    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let err = tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();

                // ...and only realized it when she tries to communicate with the server
                // (note the monitors are not running in the client, hence no risk of
                // having a concurrent processing of the loss of access)

                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap_err()
            })
            .await
            .unwrap();

            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            // TODO: what should be the error here?
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EACCES), "{}", err);
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

// TODO: Ignored test given `BadDecryption` is not currently handled in libparsec_client
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn server_block_read_but_bad_decryption(tmp_path: TmpPath, env: &TestbedEnv) {
    env.customize(|builder| {
        // No chunk&block in the local storage, so server access must occur if we
        // read the content of a file !
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
    })
    .await;

    let wksp1_id: libparsec_types::VlobID = *env.template.get_stuff("wksp1_id");
    let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(wksp1_id);

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::block_read::Req| {
            authenticated_cmds::latest::block_read::Rep::Ok {
                needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                key_index: 1,
                block: b"<dummy>".to_vec().into(),
            }
        },
        // TODO: should register a `get_keys_bundle` here
    );

    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // Do file open + close in it own dedicated thread. This is needed
            // to avoid deadlock with tokio single threaded runtime when the
            // close waits for data flush.
            let err = tokio::task::spawn_blocking(move || {
                let mut fd = std::fs::OpenOptions::new()
                    .read(true)
                    .open(mountpoint_path.join("bar.txt"))
                    .unwrap();

                // ...and only realized it when she tries to communicate with the server
                // (note the monitors are not running in the client, hence no risk of
                // having a concurrent processing of the loss of access)

                let mut buff = Vec::new();
                fd.read_to_end(&mut buff).unwrap_err()
            })
            .await
            .unwrap();

            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            // TODO: what should be the error here?
            #[cfg(not(target_os = "windows"))]
            p_assert_eq!(err.raw_os_error(), Some(libc::EACCES), "{}", err);
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
