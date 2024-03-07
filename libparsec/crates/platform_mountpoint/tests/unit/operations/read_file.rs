// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};
use tokio::io::{AsyncReadExt, AsyncSeekExt};

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
            let mut fd = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            let mut buff = Vec::new();
            fd.read_to_end(&mut buff).await.unwrap();
            p_assert_eq!(buff, b"hello world");

            // Read with cursor exhausted
            let mut buff = Vec::new();
            fd.read_to_end(&mut buff).await.unwrap();
            p_assert_eq!(buff, b"");

            // Re-wind the cursor, and read a part of the file
            fd.seek(std::io::SeekFrom::Start(3)).await.unwrap();
            let mut buff = vec![0; 6];
            fd.read_exact(&mut buff).await.unwrap();
            p_assert_eq!(buff, b"lo wor");
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn read_too_much(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            let mut buff = vec![0; 20];
            let err = fd.read_exact(&mut buff).await.unwrap_err();

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
            let mut fd = tokio::fs::OpenOptions::new()
                .write(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            let mut buff = vec![0; 20];
            let err = fd.read_exact(&mut buff).await.unwrap_err();

            p_assert_eq!(err.raw_os_error(), Some(libc::EBADF), "{}", err);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(tmp_path: TmpPath, env: &TestbedEnv) {
    mount_and_test!(
        env,
        &tmp_path,
        |client: Arc<Client>, wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            client.stop_workspace(wksp1_ops.realm_id()).await;

            let mut buff = Vec::new();
            let err = fd.read_to_end(&mut buff).await.unwrap_err();

            p_assert_eq!(err.raw_os_error(), Some(libc::EIO), "{}", err);
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(tmp_path: TmpPath, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        // No chunk&block in the local storage, so server access must occur if we
        // read the content of a file !
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
    });
    mount_and_test!(
        &env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            let mut buff = Vec::new();
            let err = fd.read_to_end(&mut buff).await.unwrap_err();
            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            p_assert_eq!(err.raw_os_error(), Some(libc::EHOSTUNREACH), "{}", err);
        }
    );
}

// TODO: Investigate why two `block_read` are send to server
// TODO: Ignored test given `NoRealmAccess` is not currently implemented in libparsec_client
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn no_realm_access(tmp_path: TmpPath, env: &TestbedEnv) {
    let env = env.customize(|builder| {
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
    });

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
        &env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            // ...and only realized it when she tries to communicate with the server
            // (note the monitors are not running in the client, hence no risk of
            // having a concurrent processing of the loss of access)

            let mut buff = Vec::new();
            let err = fd.read_to_end(&mut buff).await.unwrap_err();
            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            p_assert_eq!(err.raw_os_error(), Some(libc::EACCES), "{}", err);
        }
    );
}

// TODO: Ignored test given `StoreUnavailable` is not currently implemented in libparsec_client
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn server_block_read_but_store_unavailable(tmp_path: TmpPath, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        // No chunk&block in the local storage, so server access must occur if we
        // read the content of a file !
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
    });

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::block_read::Req| {
            authenticated_cmds::latest::block_read::Rep::StoreUnavailable
        },
    );

    mount_and_test!(
        &env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            // ...and only realized it when she tries to communicate with the server
            // (note the monitors are not running in the client, hence no risk of
            // having a concurrent processing of the loss of access)

            let mut buff = Vec::new();
            let err = fd.read_to_end(&mut buff).await.unwrap_err();
            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            // TODO: what should be the error here?
            p_assert_eq!(err.raw_os_error(), Some(libc::EACCES), "{}", err);
        }
    );
}

// TODO: Ignored test given `BadDecryption` is not currently handled in libparsec_client
#[ignore]
#[parsec_test(testbed = "minimal_client_ready")]
async fn server_block_read_but_bad_decryption(tmp_path: TmpPath, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        // No chunk&block in the local storage, so server access must occur if we
        // read the content of a file !
        builder.filter_client_storage_events(|e| {
            !matches!(
                e,
                TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
                    | TestbedEvent::WorkspaceDataStorageChunkCreate(_)
            )
        });
    });

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::block_read::Req| {
            authenticated_cmds::latest::block_read::Rep::Ok {
                block: b"<dummy>".to_vec().into(),
            }
        },
    );

    mount_and_test!(
        &env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            let mut fd = tokio::fs::OpenOptions::new()
                .read(true)
                .open(&mountpoint_path.join("bar.txt"))
                .await
                .unwrap();

            // ...and only realized it when she tries to communicate with the server
            // (note the monitors are not running in the client, hence no risk of
            // having a concurrent processing of the loss of access)

            let mut buff = Vec::new();
            let err = fd.read_to_end(&mut buff).await.unwrap_err();
            // Cannot use `std::io::ErrorKind::HostUnreachable` as it is unstable
            // TODO: what should be the error here?
            p_assert_eq!(err.raw_os_error(), Some(libc::EACCES), "{}", err);
        }
    );
}
