// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{io::Write, path::PathBuf, sync::Arc};

use libparsec_client::{workspace::WorkspaceOps, Client};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn file(
    #[values(
        "default",
        "just_created",
        "just_modified",
        "just_moved",
        "read_only_workspace"
    )]
    kind: &'static str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    if kind == "read_only_workspace" {
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
    }

    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // For performance reasons, we run all FS operations in a single thread
            // instead of using their tokio version.
            // This is needed given the OS kernel cleans the closed files after the
            // close has returned, so being fast here helps detecting concurrency issues
            // (true story).
            tokio::task::spawn_blocking(move || {
                let (file_path, expected_size) = match kind {
                    "default" | "read_only_workspace" => {
                        let path = mountpoint_path.join("bar.txt");
                        (path, 11)
                    }
                    "just_created" => {
                        let path = mountpoint_path.join("xxx.txt");
                        std::fs::File::create(&path).unwrap();
                        (path, 0)
                    }
                    "just_modified" => {
                        let path = mountpoint_path.join("bar.txt");
                        let mut fd = std::fs::OpenOptions::new()
                            .truncate(true)
                            .write(true)
                            .open(&path)
                            .unwrap();
                        fd.write_all(b"a").unwrap();
                        (path, 1)
                    }
                    "just_moved" => {
                        let path = mountpoint_path.join("xxx.txt");
                        std::fs::rename(mountpoint_path.join("bar.txt"), &path).unwrap();
                        (path, 11)
                    }
                    unknown => panic!("Unknown kind: {}", unknown),
                };
                let stat = std::fs::metadata(file_path).unwrap();
                assert!(stat.is_file());
                p_assert_eq!(stat.len(), expected_size);
            })
            .await
            .unwrap();
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn folder(
    #[values("default", "just_created", "just_moved")] kind: &'static str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    mount_and_test!(
        env,
        &tmp_path,
        |_client: Arc<Client>, _wksp1_ops: Arc<WorkspaceOps>, mountpoint_path: PathBuf| async move {
            // For performance reasons, we run all FS operations in a single thread
            // instead of using their tokio version.
            // This is needed given the OS kernel cleans the closed files after the
            // close has returned, so being fast here helps detecting concurrency issues
            // (true story).
            tokio::task::spawn_blocking(move || {
                let folder_path = match kind {
                    "default" => mountpoint_path.join("foo"),
                    "just_created" => {
                        let path = mountpoint_path.join("xxx");
                        std::fs::create_dir(&path).unwrap();
                        path
                    }
                    "just_moved" => {
                        let path = mountpoint_path.join("xxx");
                        std::fs::rename(mountpoint_path.join("foo"), &path).unwrap();
                        path
                    }
                    unknown => panic!("Unknown kind: {}", unknown),
                };
                let stat = std::fs::metadata(folder_path).unwrap();
                assert!(stat.is_dir());
            })
            .await
            .unwrap();
        }
    );
}
