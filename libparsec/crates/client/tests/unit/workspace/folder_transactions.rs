// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashSet, fmt::Display, sync::Arc};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{ls, workspace_ops_factory, workspace_ops_with_prevent_sync_pattern_factory};
use crate::{
    workspace::{EntryStat, FileStat, MoveEntryMode},
    EventWorkspaceOpsOutboundSyncNeeded, WorkspaceOps,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntryType {
    File,
    Folder,
}

impl Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::File => write!(f, "file"),
            EntryType::Folder => write!(f, "folder"),
        }
    }
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn add_confined_entry(
    #[values(true, false)] root_level: bool,
    #[values(EntryType::File, EntryType::Folder)] entry_type: EntryType,
    env: &TestbedEnv,
) {
    let Env {
        realm_id,
        parent_id,
        base_path,
    } = bootstrap_env(env, root_level).await;

    let confined_entry_name = "test.tmp".parse::<EntryName>().unwrap();
    let not_confined_entry_name = "test_not_tmp".parse::<EntryName>().unwrap();

    // Create a workspace ops instance
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, realm_id).await;
    let mut spy = ops.event_bus.spy.start_expecting();

    // Check that parent is up to date
    check_need_sync_parent(&ops, parent_id, false).await;
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 0);

    // Create the confined child
    let target = base_path.join(confined_entry_name.clone());
    let confined_child_id = match entry_type {
        EntryType::File => ops.create_file(target).await.unwrap(),
        EntryType::Folder => ops.create_folder(target).await.unwrap(),
    };

    // Check that the child is marked as needing sync (this is because the child itself
    // is not aware of its name and can be renamed at anytime, so it's not its job to
    // decide whether or not it should be synced)
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, realm_id);
        p_assert_eq!(e.entry_id, confined_child_id);
    });
    drop(spy);
    check_child(
        &ops,
        entry_type,
        confined_child_id,
        ExpectedValues {
            need_sync: true,
            confinement_point: Some(parent_id),
        },
    )
    .await;

    // Check that parent is not marked as needing sync
    check_need_sync_parent(&ops, parent_id, false).await;

    // Check that the child is in the parent's children
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 1);
    let (name, stat) = &children[0];
    p_assert_eq!(name, &confined_entry_name);
    check_stat(
        entry_type,
        confined_child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: Some(parent_id),
        },
    );

    // Check parent manifest content
    let parent_manifest = ops
        .store
        .get_manifest(parent_id)
        .await
        .expect("unable to retrieve local manifest");
    if let ArcLocalChildManifest::Folder(folder_manifest) = parent_manifest {
        assert!(folder_manifest
            .local_confinement_points
            .contains(&confined_child_id));
    } else {
        panic!("parent expected to be a folder")
    }

    // Create a non-confined file
    let target = base_path.join("test_not_tmp".parse().unwrap());
    let not_confined_child_id = match entry_type {
        EntryType::File => ops.create_file(target).await.unwrap(),
        EntryType::Folder => ops.create_folder(target).await.unwrap(),
    };
    // Check that parent is marked as needing sync
    check_need_sync_parent(&ops, parent_id, true).await;

    // Check that both children are in the parent's children
    let mut children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 2);
    children.sort_by(|(a, _), (b, _)| a.cmp(b));
    let (name, stat) = &children[0];
    p_assert_eq!(name, &confined_entry_name);
    check_stat(
        entry_type,
        confined_child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: Some(parent_id),
        },
    );
    let (name, stat) = &children[1];
    p_assert_eq!(name, &not_confined_entry_name);
    check_stat(
        entry_type,
        not_confined_child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: None,
        },
    );

    // Check parent manifest content
    let parent_manifest = ops
        .store
        .get_manifest(parent_id)
        .await
        .expect("unable to retrieve local manifest");
    if let ArcLocalChildManifest::Folder(folder_manifest) = parent_manifest {
        assert!(folder_manifest
            .local_confinement_points
            .contains(&confined_child_id));
        assert!(!folder_manifest
            .local_confinement_points
            .contains(&not_confined_child_id));
    } else {
        panic!("parent expected to be a folder")
    }

    // Outbound sync
    ops_outbound_sync(&ops).await;

    // Check that parent is not marked as needing sync
    check_need_sync_parent(&ops, parent_id, false).await;

    // Check that both children are in the parent's children
    let mut children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 2);
    children.sort_by(|(a, _), (b, _)| a.cmp(b));
    let (name, stat) = &children[0];
    p_assert_eq!(name, &confined_entry_name);
    check_stat(
        entry_type,
        confined_child_id,
        stat,
        ExpectedValues {
            // TODO: need_sync should be true, i.e the confined file should not have been synced
            // See: https://github.com/Scille/parsec-cloud/issues/8198
            need_sync: false,
            confinement_point: Some(parent_id),
        },
    );
    let (name, stat) = &children[1];
    p_assert_eq!(name, &not_confined_entry_name);
    check_stat(
        entry_type,
        not_confined_child_id,
        stat,
        ExpectedValues {
            need_sync: false,
            confinement_point: None,
        },
    );

    // Check parent manifest content
    let parent_manifest = ops
        .store
        .get_manifest(parent_id)
        .await
        .expect("unable to retrieve local manifest");
    if let ArcLocalChildManifest::Folder(folder_manifest) = parent_manifest {
        assert!(folder_manifest
            .local_confinement_points
            .contains(&confined_child_id));
        assert!(!folder_manifest
            .local_confinement_points
            .contains(&not_confined_child_id));
        assert!(!folder_manifest
            .remote_confinement_points
            .contains(&confined_child_id));
        assert!(!folder_manifest
            .remote_confinement_points
            .contains(&not_confined_child_id));
    } else {
        panic!("parent expected to be a folder")
    }
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn remove_confined_entry(
    #[values(true, false)] root_level: bool,
    #[values(EntryType::File, EntryType::Folder)] entry_type: EntryType,
    env: &TestbedEnv,
) {
    let Env {
        realm_id,
        parent_id,
        base_path,
    } = bootstrap_env(env, root_level).await;

    // Create a workspace ops instance
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, realm_id.to_owned()).await;
    let mut spy = ops.event_bus.spy.start_expecting();

    // Check that parent is up to date
    check_need_sync_parent(&ops, parent_id, false).await;
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 0);

    // Create the confined child
    let target = base_path.join("test.tmp".parse().unwrap());
    let confined_child_id = match entry_type {
        EntryType::File => ops.create_file(target.clone()).await.unwrap(),
        EntryType::Folder => ops.create_folder(target.clone()).await.unwrap(),
    };

    // Check that the child is marked as needing sync (this is because the child itself
    // is not aware of its name and can be renamed at anytime, so it's not its job to
    // decide whether or not it should be synced)
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, realm_id);
        p_assert_eq!(e.entry_id, confined_child_id);
    });
    check_child(
        &ops,
        entry_type,
        confined_child_id,
        ExpectedValues {
            need_sync: true,
            confinement_point: Some(parent_id),
        },
    )
    .await;

    // Check that parent is not marked as needing sync
    check_need_sync_parent(&ops, parent_id, false).await;

    // Check that the child is in the parent's children
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 1);
    let (name, stat) = children[0].clone();
    p_assert_eq!(name, "test.tmp".parse::<EntryName>().unwrap());
    check_stat(
        entry_type,
        confined_child_id,
        &stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: Some(parent_id),
        },
    );

    // Check parent manifest content
    let parent_manifest = ops
        .store
        .get_manifest(parent_id)
        .await
        .expect("unable to retrieve local manifest");

    if let ArcLocalChildManifest::Folder(folder_manifest) = parent_manifest {
        assert_eq!(
            folder_manifest.local_confinement_points,
            HashSet::from([confined_child_id])
        );
    } else {
        panic!("parent expected to be a folder")
    }

    // Remove child
    ops.remove_entry(target).await.unwrap();

    // Parent must be updated
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, realm_id);
        p_assert_eq!(e.entry_id, parent_id);
    });

    // Check that the child is NOT in the parent's children
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    assert!(children.is_empty());

    // Check parent manifest content
    let parent_manifest = ops
        .store
        .get_manifest(parent_id)
        .await
        .expect("unable to retrieve local manifest");
    if let ArcLocalChildManifest::Folder(folder_manifest) = parent_manifest {
        assert!(folder_manifest.local_confinement_points.is_empty())
    } else {
        panic!("parent expected to be a folder")
    }
}

struct Env {
    realm_id: VlobID,
    parent_id: VlobID,
    base_path: FsPath,
}

async fn bootstrap_env(env: &TestbedEnv, root_level: bool) -> Env {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let (base_path, parent_id) = env
        .customize(|builder| {
            let prevent_sync_pattern = Regex::from_regex_str(r"\.tmp$").unwrap();
            let res = if root_level {
                // Remove all children from the workspace
                builder
                    .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.manifest);
                        manifest.children.clear();
                    });
                ("/".parse().unwrap(), wksp1_id)
            } else {
                let wksp1_foo_id = builder.counters.next_entry_id();
                // Create foo folder
                builder
                    .create_or_update_folder_manifest_vlob(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_foo_id,
                        wksp1_id,
                    )
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.manifest);
                        manifest.children.clear();
                    });
                builder.workspace_data_storage_fetch_folder_vlob(
                    "alice@dev1",
                    wksp1_id,
                    wksp1_foo_id,
                    prevent_sync_pattern.clone(),
                );
                // Add foo folder to workspace
                builder
                    .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
                    .customize(|e| {
                        let manifest = Arc::make_mut(&mut e.manifest);
                        manifest
                            .children
                            .insert("foo".parse().unwrap(), wksp1_foo_id);
                    });
                ("/foo".parse().unwrap(), wksp1_foo_id)
            };
            // Fetch the workspace data
            builder.workspace_data_storage_fetch_workspace_vlob(
                "alice@dev1",
                wksp1_id,
                prevent_sync_pattern,
            );
            res
        })
        .await;

    Env {
        base_path,
        parent_id,
        realm_id: wksp1_id,
    }
}

struct ExpectedValues {
    need_sync: bool,
    confinement_point: Option<VlobID>,
}

#[track_caller]
fn check_child(
    ops: &'_ WorkspaceOps,
    entry_type: EntryType,
    id: VlobID,
    expected: ExpectedValues,
) -> impl std::future::Future<Output = ()> + '_ {
    let caller = std::panic::Location::caller();
    async move {
        let child_stat = ops.stat_entry_by_id(id).await.unwrap();
        check_stat_with_caller(entry_type, id, &child_stat, expected, caller)
    }
}

#[track_caller]
fn check_stat(entry_type: EntryType, id: VlobID, stat: &EntryStat, expected: ExpectedValues) {
    let caller = std::panic::Location::caller();
    check_stat_with_caller(entry_type, id, stat, expected, caller)
}

fn check_stat_with_caller(
    entry_type: EntryType,
    id: VlobID,
    stat: &EntryStat,
    expected: ExpectedValues,
    caller: &std::panic::Location<'static>,
) {
    let (got_id, need_sync, confinement_point) = match (entry_type, stat) {
        (
            EntryType::File,
            EntryStat::File {
                confinement_point,
                base: FileStat { id, need_sync, .. },
                ..
            },
        )
        | (
            EntryType::Folder,
            EntryStat::Folder {
                id,
                need_sync,
                confinement_point,
                ..
            },
        ) => (id, need_sync, confinement_point),
        _ => panic!("Expected a {} (caller: {})", entry_type, caller),
    };
    p_assert_eq!(got_id, &id, "Invalid id in EntryStat (caller: {})", caller);
    p_assert_eq!(
        *need_sync,
        expected.need_sync,
        "Invalid need_sync (caller: {})",
        caller
    );
    p_assert_eq!(
        confinement_point,
        &expected.confinement_point,
        "Invalid confinement point (caller: {})",
        caller
    );
}

#[track_caller]
fn check_need_sync_parent(
    ops: &'_ WorkspaceOps,
    id: VlobID,
    expected: bool,
) -> impl std::future::Future<Output = ()> + '_ {
    let caller = std::panic::Location::caller();
    async move {
        let parent_stat = ops.stat_entry_by_id(id).await.unwrap();
        check_stat_with_caller(
            EntryType::Folder,
            id,
            &parent_stat,
            ExpectedValues {
                need_sync: expected,
                confinement_point: None,
            },
            caller,
        );
    }
}

async fn ops_outbound_sync(ops: &WorkspaceOps) {
    loop {
        let entries = ops.get_need_outbound_sync(32).await.unwrap();
        if entries.is_empty() {
            break;
        }
        for entry in entries {
            ops.outbound_sync(entry).await.unwrap();
        }
    }
}

async fn rename_same_parent(
    flavor: RenameFlavor,
    ops: Arc<WorkspaceOps>,
    parent_id: VlobID,
    parent_path: FsPath,
    src: EntryName,
    dst: EntryName,
) {
    match flavor {
        RenameFlavor::RenameEntry => ops
            .rename_entry_by_id(parent_id, src, dst, MoveEntryMode::CanReplace)
            .await
            .unwrap(),
        RenameFlavor::MoveEntry => ops
            .move_entry(
                parent_path.join(src),
                parent_path.join(dst),
                MoveEntryMode::CanReplace,
            )
            .await
            .unwrap(),
    }
}

enum RenameFlavor {
    RenameEntry,
    MoveEntry,
}

/// Test that renaming a file or a folder to match the prevent sync pattern make the file disappear from the parent in the remote
#[parsec_test(testbed = "coolorg", with_server)]
async fn rename_a_entry_to_be_confined(
    #[values(RenameFlavor::RenameEntry, RenameFlavor::MoveEntry)] rename_flavor: RenameFlavor,
    #[values(true, false)] root_level: bool,
    #[values(EntryType::File, EntryType::Folder)] entry_type: EntryType,
    env: &TestbedEnv,
) {
    let Env {
        realm_id,
        parent_id,
        base_path,
    } = bootstrap_env(env, root_level).await;

    let confined_entry_name = "test.tmp".parse::<EntryName>().unwrap();
    let not_confined_entry_name = "test_not_tmp".parse::<EntryName>().unwrap();

    // Create a workspace ops instance
    let alice = env.local_device("alice@dev1");
    let ops = Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, realm_id).await);

    // Check that parent is up to date
    check_need_sync_parent(&ops, parent_id, false).await;
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 0);

    // Create a file that is not confined
    let target = base_path.join(not_confined_entry_name.clone());
    let child_id = match entry_type {
        EntryType::File => ops.create_file(target).await.unwrap(),
        EntryType::Folder => ops.create_folder(target).await.unwrap(),
    };

    // Check that the child is in the parent's children
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 1);
    let (name, stat) = &children[0];
    p_assert_eq!(name, &not_confined_entry_name);
    check_stat(
        entry_type,
        child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: None,
        },
    );

    // Check that parent is marked as needing sync
    check_need_sync_parent(&ops, parent_id, true).await;

    // Rename the file to be confined
    rename_same_parent(
        rename_flavor,
        ops.clone(),
        parent_id,
        base_path,
        not_confined_entry_name.clone(),
        confined_entry_name.clone(),
    )
    .await;

    // Check that the child is renamed and need sync
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 1);
    let (name, stat) = &children[0];
    p_assert_eq!(name, &confined_entry_name);
    check_stat(
        entry_type,
        child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: Some(parent_id),
        },
    );

    // Check that parent is still marked as needing sync
    check_need_sync_parent(&ops, parent_id, true).await;

    let ArcLocalChildManifest::Folder(folder) = ops.store.get_manifest(parent_id).await.unwrap()
    else {
        panic!("Expected a folder");
    };
    assert!(
        folder.local_confinement_points.contains(&child_id),
        "The child {} should be in the confinement points list",
        child_id
    );

    ops_outbound_sync(&ops).await;

    let bob = env.local_device("bob@dev1");
    let bob_ops = workspace_ops_with_prevent_sync_pattern_factory(
        &env.discriminant_dir,
        &bob,
        realm_id,
        Regex::empty(), // Use empty pattern to prevent workspace to filter out any entry
    )
    .await;
    ops_inbound_sync(&bob_ops).await;

    // Check that the child is no longer present on the remote
    let ArcLocalChildManifest::Folder(folder) =
        bob_ops.store.get_manifest(parent_id).await.unwrap()
    else {
        panic!("Expected a folder");
    };
    assert!(
        dbg!(&folder.children).is_empty(),
        "{} should have been removed with the rename",
        not_confined_entry_name
    );
}

async fn ops_inbound_sync(ops: &WorkspaceOps) {
    ops.refresh_realm_checkpoint().await.unwrap();
    loop {
        let entries = ops.get_need_inbound_sync(32).await.unwrap();
        if entries.is_empty() {
            break;
        }
        for entry in entries {
            ops.inbound_sync(entry).await.unwrap();
        }
    }
}

/// Test that renaming a file or a folder that was confined make it available on remote
#[parsec_test(testbed = "coolorg", with_server)]
async fn rename_a_confined_entry_to_not_be_confined(
    #[values(RenameFlavor::RenameEntry, RenameFlavor::MoveEntry)] rename_flavor: RenameFlavor,
    #[values(true, false)] root_level: bool,
    #[values(EntryType::File, EntryType::Folder)] entry_type: EntryType,
    env: &TestbedEnv,
) {
    let Env {
        realm_id,
        parent_id,
        base_path,
    } = bootstrap_env(env, root_level).await;

    let confined_entry_name = "test.tmp".parse::<EntryName>().unwrap();
    let not_confined_entry_name = "test_not_tmp".parse::<EntryName>().unwrap();

    // Create a workspace ops instance
    let alice = env.local_device("alice@dev1");
    let ops = Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, realm_id).await);

    // Check that parent is up to date
    check_need_sync_parent(&ops, parent_id, false).await;
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 0);

    // Create a file that is not confined
    let target = base_path.join(confined_entry_name.clone());
    let child_id = match entry_type {
        EntryType::File => ops.create_file(target).await.unwrap(),
        EntryType::Folder => ops.create_folder(target).await.unwrap(),
    };

    // Create a file that is not confined so the parent is marked as needing sync
    let target = base_path.join("file-to-sync".parse().unwrap());
    ops.create_folder(target).await.unwrap();

    // Check that the child is in the parent's children
    let children = dbg!(ops.stat_folder_children_by_id(parent_id).await.unwrap());
    p_assert_eq!(children.len(), 2);
    let (name, stat) = children
        .iter()
        .find(|(name, _)| name == &confined_entry_name)
        .unwrap();
    p_assert_eq!(name, &confined_entry_name);
    check_stat(
        entry_type,
        child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: Some(parent_id),
        },
    );

    // Check that parent is marked as needing sync
    check_need_sync_parent(&ops, parent_id, true).await;

    // Rename the file to a non-confined name
    rename_same_parent(
        rename_flavor,
        ops.clone(),
        parent_id,
        base_path,
        confined_entry_name.clone(),
        not_confined_entry_name.clone(),
    )
    .await;

    // Check that the child is renamed and need sync
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 2);
    let (name, stat) = children
        .iter()
        .find(|(name, _)| name == &not_confined_entry_name)
        .unwrap();
    p_assert_eq!(name, &not_confined_entry_name);
    check_stat(
        entry_type,
        child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: None,
        },
    );

    // Check that parent is still marked as needing sync
    check_need_sync_parent(&ops, parent_id, true).await;

    let ArcLocalChildManifest::Folder(folder) = ops.store.get_manifest(parent_id).await.unwrap()
    else {
        panic!("Expected a folder");
    };
    assert!(
        folder.local_confinement_points.is_empty(),
        "The child {} should not be in the confinement points list",
        child_id
    );

    ops_outbound_sync(&ops).await;

    let bob = env.local_device("bob@dev1");
    let bob_ops = workspace_ops_with_prevent_sync_pattern_factory(
        &env.discriminant_dir,
        &bob,
        realm_id,
        Regex::empty(), // Use empty pattern to prevent workspace to filter out any entry
    )
    .await;
    ops_inbound_sync(&bob_ops).await;

    // Check that the child is no longer present on the remote
    let ArcLocalChildManifest::Folder(folder) =
        bob_ops.store.get_manifest(parent_id).await.unwrap()
    else {
        panic!("Expected a folder");
    };
    assert_eq!(
        folder.children[&not_confined_entry_name], child_id,
        "{} should be present in the folder",
        not_confined_entry_name
    );

    // Ensure that the child is present in the remote
    let child_manifest = bob_ops.store.get_manifest(child_id).await.unwrap();
    assert_eq!(child_manifest.parent(), parent_id);
}

/// Test that renaming a file or a folder to match the prevent sync pattern make the file disappear from the parent in the remote
#[parsec_test(testbed = "coolorg", with_server)]
async fn rename_a_entry_to_be_confined_with_different_parent(
    #[values(true, false)] root_level: bool,
    #[values(EntryType::File, EntryType::Folder)] entry_type: EntryType,
    env: &TestbedEnv,
) {
    let Env {
        realm_id,
        parent_id,
        base_path,
    } = bootstrap_env(env, root_level).await;

    let confined_entry_name = "test.tmp".parse::<EntryName>().unwrap();
    let not_confined_entry_name = "test_not_tmp".parse::<EntryName>().unwrap();

    // Create a workspace ops instance
    let alice = env.local_device("alice@dev1");
    let ops = Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, realm_id).await);

    // Check that parent is up to date
    check_need_sync_parent(&ops, parent_id, false).await;
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 0);

    // Create a folder that is not confined so the parent is marked as needing sync
    let different_parent_name = "bar".parse::<EntryName>().unwrap();
    let different_parent_path = base_path.join(different_parent_name.clone());
    let different_parent_id = ops
        .create_folder(different_parent_path.clone())
        .await
        .unwrap();

    // Create a file that is not confined
    let target = base_path.join(not_confined_entry_name.clone());
    let child_id = match entry_type {
        EntryType::File => ops.create_file(target).await.unwrap(),
        EntryType::Folder => ops.create_folder(target).await.unwrap(),
    };

    // Check that the child is in the parent's children
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 2);
    let (name, stat) = children
        .iter()
        .find(|(name, _)| name == &not_confined_entry_name)
        .unwrap();
    p_assert_eq!(name, &not_confined_entry_name);
    check_stat(
        entry_type,
        child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: None,
        },
    );

    // Check that parent is marked as needing sync
    check_need_sync_parent(&ops, parent_id, true).await;

    // Rename the file to be confined
    ops.move_entry(
        base_path.join(not_confined_entry_name.clone()),
        different_parent_path.join(confined_entry_name.clone()),
        MoveEntryMode::NoReplace,
    )
    .await
    .unwrap();

    // Check that the child is renamed and need sync
    let children = ops
        .stat_folder_children_by_id(different_parent_id)
        .await
        .unwrap();
    p_assert_eq!(children.len(), 1);
    let (name, stat) = &children[0];
    p_assert_eq!(name, &confined_entry_name);
    check_stat(
        entry_type,
        child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: Some(different_parent_id),
        },
    );

    // Check that parent is still marked as needing sync
    check_need_sync_parent(&ops, parent_id, true).await;
    check_need_sync_parent(&ops, different_parent_id, true).await;

    let ArcLocalChildManifest::Folder(folder) =
        ops.store.get_manifest(different_parent_id).await.unwrap()
    else {
        panic!("Expected a folder");
    };
    assert!(
        folder.local_confinement_points.contains(&child_id),
        "The child {} should be in the confinement points list",
        child_id
    );

    ops_outbound_sync(&ops).await;

    let bob = env.local_device("bob@dev1");
    let bob_ops = workspace_ops_with_prevent_sync_pattern_factory(
        &env.discriminant_dir,
        &bob,
        realm_id,
        Regex::empty(), // Use empty pattern to prevent workspace to filter out any entry
    )
    .await;
    ops_inbound_sync(&bob_ops).await;

    // Check that the child is no longer present on the remote
    let ArcLocalChildManifest::Folder(folder) =
        bob_ops.store.get_manifest(parent_id).await.unwrap()
    else {
        panic!("Expected a folder");
    };
    assert_eq!(
        dbg!(&folder.children).len(),
        1,
        "Should only have {}, {} should have been removed with the rename",
        different_parent_id,
        not_confined_entry_name
    );
    assert_eq!(folder.children[&different_parent_name], different_parent_id);
    // The different parent should also be empty since the renamed file match the prevent sync pattern.
    let ArcLocalChildManifest::Folder(folder) = bob_ops
        .store
        .get_manifest(different_parent_id)
        .await
        .unwrap()
    else {
        panic!("Expected a folder");
    };
    assert!(
        dbg!(&folder.children).is_empty(),
        "{} should have been removed with the rename",
        not_confined_entry_name
    );
}

/// Test that renaming a file or a folder that was confined make it available on remote
#[parsec_test(testbed = "coolorg", with_server)]
async fn rename_a_confined_entry_to_not_be_confined_with_different_parent(
    #[values(true, false)] root_level: bool,
    #[values(EntryType::File, EntryType::Folder)] entry_type: EntryType,
    env: &TestbedEnv,
) {
    let Env {
        realm_id,
        parent_id,
        base_path,
    } = bootstrap_env(env, root_level).await;

    let confined_entry_name = "test.tmp".parse::<EntryName>().unwrap();
    let not_confined_entry_name = "test_not_tmp".parse::<EntryName>().unwrap();

    // Create a workspace ops instance
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, realm_id).await;

    // Check that parent is up to date
    check_need_sync_parent(&ops, parent_id, false).await;
    let children = ops.stat_folder_children_by_id(parent_id).await.unwrap();
    p_assert_eq!(children.len(), 0);

    // Create a file that is not confined
    let target = base_path.join(confined_entry_name.clone());
    let child_id = match entry_type {
        EntryType::File => ops.create_file(target).await.unwrap(),
        EntryType::Folder => ops.create_folder(target).await.unwrap(),
    };

    // Create a folder that is not confined so the parent is marked as needing sync
    let different_parent_name = "bar".parse::<EntryName>().unwrap();
    let different_parent_path = base_path.join(different_parent_name.clone());
    let different_parent_id = ops
        .create_folder(different_parent_path.clone())
        .await
        .unwrap();

    // Check that the child is in the parent's children
    let children = dbg!(ops.stat_folder_children_by_id(parent_id).await.unwrap());
    p_assert_eq!(children.len(), 2);
    let (name, stat) = children
        .iter()
        .find(|(name, _)| name == &confined_entry_name)
        .unwrap();
    p_assert_eq!(name, &confined_entry_name);
    check_stat(
        entry_type,
        child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: Some(parent_id),
        },
    );

    // Check that parent is marked as needing sync
    check_need_sync_parent(&ops, parent_id, true).await;

    // Rename the file to a non-confined name
    ops.move_entry(
        base_path.join(confined_entry_name.clone()),
        different_parent_path.join(not_confined_entry_name.clone()),
        MoveEntryMode::CanReplace,
    )
    .await
    .unwrap();

    // Check that the child is renamed and need sync
    let children = ops
        .stat_folder_children_by_id(different_parent_id)
        .await
        .unwrap();
    p_assert_eq!(children.len(), 1);
    let (name, stat) = &children[0];
    p_assert_eq!(name, &not_confined_entry_name);
    check_stat(
        entry_type,
        child_id,
        stat,
        ExpectedValues {
            need_sync: true,
            confinement_point: None,
        },
    );

    // Check that parent is still marked as needing sync
    check_need_sync_parent(&ops, parent_id, true).await;
    check_need_sync_parent(&ops, different_parent_id, true).await;

    let ArcLocalChildManifest::Folder(folder) =
        ops.store.get_manifest(different_parent_id).await.unwrap()
    else {
        panic!("Expected a folder");
    };
    assert!(
        folder.local_confinement_points.is_empty(),
        "The child {} should not be in the confinement points list",
        child_id
    );

    ops_outbound_sync(&ops).await;

    let bob = env.local_device("bob@dev1");
    let bob_ops = workspace_ops_with_prevent_sync_pattern_factory(
        &env.discriminant_dir,
        &bob,
        realm_id,
        Regex::empty(), // Use empty pattern to prevent workspace to filter out any entry
    )
    .await;
    ops_inbound_sync(&bob_ops).await;

    // Check that the child is present on the remote in different_parent
    let ArcLocalChildManifest::Folder(folder) = bob_ops
        .store
        .get_manifest(different_parent_id)
        .await
        .unwrap()
    else {
        panic!("Expected a folder");
    };
    assert_eq!(
        folder.children[&not_confined_entry_name], child_id,
        "{} should be present in the folder",
        not_confined_entry_name
    );
    // But should have been removed from the parent
    let ArcLocalChildManifest::Folder(folder) =
        bob_ops.store.get_manifest(parent_id).await.unwrap()
    else {
        panic!("Expected a folder");
    };
    assert_eq!(
        dbg!(&folder.children).len(),
        1,
        "{} Should only contain {}",
        parent_id,
        different_parent_id
    );
    assert_eq!(folder.children[&different_parent_name], different_parent_id);

    // Ensure that the child is present in the remote
    let child_manifest = bob_ops.store.get_manifest(child_id).await.unwrap();
    assert_eq!(child_manifest.parent(), different_parent_id);
}
