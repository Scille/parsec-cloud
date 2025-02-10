// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{os::unix::fs::PermissionsExt, path::PathBuf, sync::Arc};

use libparsec_client::{workspace::WorkspaceOps, Client};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::mount_and_test;

#[parsec_test(testbed = "minimal_client_ready")]
async fn file(
    #[values("default", "read_only_workspace")] access_rights: &'static str,
    #[values("file", "folder")] entry_type: &'static str,
    tmp_path: TmpPath,
    env: &TestbedEnv,
) {
    let expected_permission = match access_rights {
        "read_only_workspace" => {
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
            0o500
        }
        _ => 0o700,
    };

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
                let path = match entry_type {
                    "file" => mountpoint_path.join("bar.txt"),
                    "folder" => mountpoint_path.join("foo"),
                    _ => panic!("Invalid entry type"),
                };
                let stat = std::fs::metadata(path).unwrap();
                match entry_type {
                    "file" => assert!(stat.is_file()),
                    "folder" => assert!(stat.is_dir()),
                    _ => panic!("Invalid entry type"),
                }
                p_assert_eq!(stat.permissions().mode() & 0o777, expected_permission);
            })
            .await
            .unwrap();
        }
    );
}
