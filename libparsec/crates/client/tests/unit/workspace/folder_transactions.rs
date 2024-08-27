// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{ls, workspace_ops_factory};
use crate::{
    workspace::{EntryStat, MoveEntryMode},
    EventWorkspaceOpsOutboundSyncNeeded,
};

#[parsec_test(testbed = "minimal_client_ready")]
async fn good(#[values(true, false)] root_level: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    // Remove any sub file/folder from the place we are going to test from
    env.customize(|builder| {
        if root_level {
            builder
                .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.manifest);
                    manifest.children.clear();
                });
            builder.workspace_data_storage_fetch_workspace_vlob(
                "alice@dev1",
                wksp1_id,
                libparsec_types::Regex::empty(),
            );
        } else {
            builder
                .create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, wksp1_foo_id, None)
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.manifest);
                    manifest.children.clear();
                });
            builder.workspace_data_storage_fetch_folder_vlob(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                libparsec_types::Regex::empty(),
            );
        }
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;
    let mut spy = ops.event_bus.spy.start_expecting();

    let base_path: FsPath = if root_level {
        "/".parse().unwrap()
    } else {
        "/foo".parse().unwrap()
    };

    // Now let's add folders !

    let dir1 = base_path.join("dir1".parse().unwrap());
    let dir2 = base_path.join("dir2".parse().unwrap());
    let dir3 = base_path.join("dir3".parse().unwrap());

    let base_path_str = base_path.to_string();
    let dir1_str = dir1.to_string();
    let dir2_str = dir2.to_string();
    let dir3_str = dir3.to_string();

    macro_rules! create_folder {
        ($path:expr, $parent_id: ident) => {
            async {
                let child_id = ops.create_folder($path.clone()).await.unwrap();
                spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
                    p_assert_eq!(e.realm_id, wksp1_id);
                    p_assert_eq!(e.entry_id, child_id);
                });
                spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
                    p_assert_eq!(e.realm_id, wksp1_id);
                    p_assert_eq!(e.entry_id, $parent_id);
                });
                child_id
            }
        };
    }
    let parent_id = if root_level { wksp1_id } else { wksp1_foo_id };
    let dir1_id = create_folder!(dir1, parent_id).await;
    create_folder!(dir2, parent_id).await;
    let dir3_id = create_folder!(dir3, parent_id).await;

    // Add subdirs to know which dir is which once we start renaming them
    create_folder!(dir1.join("subdir1".parse().unwrap(),), dir1_id).await;
    create_folder!(dir3.join("subdir3".parse().unwrap(),), dir3_id).await;

    p_assert_eq!(ls!(ops, &base_path_str).await, ["dir1", "dir2", "dir3",]);
    p_assert_eq!(ls!(ops, &dir1_str).await, ["subdir1"]);
    p_assert_eq!(ls!(ops, &dir2_str).await, Vec::<String>::new());
    p_assert_eq!(ls!(ops, &dir3_str).await, ["subdir3"]);

    // Remove folder

    ops.remove_folder(dir2.clone()).await.unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, parent_id);
    });

    p_assert_eq!(ls!(ops, &base_path_str).await, ["dir1", "dir3"]);
    p_assert_eq!(ls!(ops, &dir1_str).await, ["subdir1"]);
    p_assert_eq!(ls!(ops, &dir3_str).await, ["subdir3"]);

    // Rename folder

    ops.move_entry(dir1, dir2.clone(), MoveEntryMode::NoReplace)
        .await
        .unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, parent_id);
    });

    p_assert_eq!(ls!(ops, &base_path_str).await, ["dir2", "dir3"]);
    p_assert_eq!(ls!(ops, &dir2_str).await, ["subdir1"]);
    p_assert_eq!(ls!(ops, &dir3_str).await, ["subdir3"]);

    // Overwrite by rename folder

    ops.move_entry(dir3, dir2, MoveEntryMode::CanReplace)
        .await
        .unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, parent_id);
    });

    p_assert_eq!(ls!(ops, &base_path_str).await, ["dir2"]);
    p_assert_eq!(ls!(ops, &dir2_str).await, ["subdir3"]);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn add_temporary_entry(
    #[values(true, false)] root_level: bool,
    #[values("file", "folder")] entry_type: &'static str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    // Remove any sub file/folder from the place we are going to test from
    env.customize(|builder| {
        if root_level {
            builder
                .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.manifest);
                    manifest.children.clear();
                });
            builder.workspace_data_storage_fetch_workspace_vlob(
                "alice@dev1",
                wksp1_id,
                Regex::from_regex_str("\\.tmp").unwrap(),
            );
        } else {
            builder
                .create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, wksp1_foo_id, None)
                .customize(|e| {
                    let manifest = Arc::make_mut(&mut e.manifest);
                    manifest.children.clear();
                });
            builder.workspace_data_storage_fetch_folder_vlob(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                Regex::from_regex_str("\\.tmp").unwrap(),
            );
        }
    })
    .await;

    // Choose parent depending on the root level
    let base_path: FsPath = if root_level {
        "/".parse().unwrap()
    } else {
        "/foo".parse().unwrap()
    };
    let parent_id = if root_level { wksp1_id } else { wksp1_foo_id };

    // Create a workspace ops instance
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;
    let mut spy = ops.event_bus.spy.start_expecting();

    // Check that parent is up to date
    let parent_stat = ops.stat_entry_by_id(parent_id).await.unwrap();
    match parent_stat {
        EntryStat::Folder { need_sync, .. } => {
            p_assert_eq!(need_sync, false);
        }
        _ => panic!("Expected a folder"),
    }

    // Create a temporary directory
    let target = base_path.join("test.tmp".parse().unwrap());
    let child_id = match entry_type {
        "file" => ops.create_file(target).await.unwrap(),
        "folder" => ops.create_folder(target).await.unwrap(),
        _ => panic!("Invalid entry type"),
    };

    // Check that the child is marked as needing sync
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, child_id);
    });
    let child_stat = ops.stat_entry_by_id(child_id).await.unwrap();
    match (entry_type, child_stat) {
        ("file", EntryStat::File { need_sync, .. }) => {
            p_assert_eq!(need_sync, true);
        }
        ("folder", EntryStat::Folder { need_sync, .. }) => {
            p_assert_eq!(need_sync, true);
        }
        _ => panic!("Expected a {}", entry_type),
    }

    // Check that parent is not marked as needing sync
    let parent_stat = ops.stat_entry_by_id(parent_id).await.unwrap();
    match parent_stat {
        EntryStat::Folder { need_sync, .. } => {
            p_assert_eq!(need_sync, false);
        }
        _ => panic!("Expected a folder"),
    }
}
