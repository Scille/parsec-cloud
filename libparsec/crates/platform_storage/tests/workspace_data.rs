// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use libparsec_platform_storage::workspace::{
    workspace_storage_non_speculative_init, NeedSyncEntries, WorkspaceDataStorage,
};

/// Due to implementation detail, entries in local need sync may be duplicated
/// (once fetched from db, once fetched from the cache), this expected and ok
///
/// Hence this helper that strip duplication before doing the compare
fn assert_need_sync(
    need_sync: NeedSyncEntries,
    expected_local: &[VlobID],
    expected_remote: &[VlobID],
) {
    let expected_local = HashSet::<_>::from_iter(expected_local);
    let expected_remote = HashSet::<_>::from_iter(expected_remote);

    let NeedSyncEntries { local, remote } = need_sync;
    let local = HashSet::from_iter(&local);
    let remote = HashSet::from_iter(&remote);

    p_assert_eq!(local, expected_local, "Local need sync");
    p_assert_eq!(remote, expected_remote, "Remote need sync");
}

#[parsec_test(testbed = "minimal")]
#[case::local_change(true)]
#[case::no_local_change(false)]
async fn get_need_sync_entries_at_startup(#[case] with_local_changes: bool, env: &TestbedEnv) {
    let mut init_realm_id = None;
    let folder_child_id = VlobID::from_hex("aa0000000000000000000000000f01de").unwrap();
    let file_child_id = VlobID::from_hex("aa00000000000000000000000000f11e").unwrap();

    let env = env.customize(|builder| {
        let realm_id = builder.new_realm("alice").map(|e| e.realm_id);
        init_realm_id = Some(realm_id);

        // Storage has fetched workspace&children manifests v1...

        builder.create_or_update_workspace_manifest_vlob("alice@dev1", realm_id);
        builder.create_or_update_folder_manifest_vlob("alice@dev1", realm_id, folder_child_id);
        builder.create_or_update_file_manifest_vlob("alice@dev1", realm_id, file_child_id);

        builder.workspace_data_storage_fetch_workspace_vlob("alice@dev1", realm_id, None);
        builder.workspace_data_storage_fetch_folder_vlob(
            "alice@dev1",
            realm_id,
            folder_child_id,
            None,
        );
        builder.workspace_data_storage_fetch_file_vlob("alice@dev1", realm_id, file_child_id);

        // ...is aware of remote change workspace&children manifest v2...

        builder.create_or_update_workspace_manifest_vlob("alice@dev1", realm_id);
        builder.create_or_update_folder_manifest_vlob("alice@dev1", realm_id, folder_child_id);
        builder.create_or_update_file_manifest_vlob("alice@dev1", realm_id, file_child_id);

        // ...and maybe contains modified versions of workspace&children

        if with_local_changes {
            builder.workspace_data_storage_local_workspace_manifest_update("alice@dev1", realm_id);
            builder.workspace_data_storage_local_file_manifest_create_or_update(
                "alice@dev1",
                realm_id,
                file_child_id,
            );
            builder.workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                realm_id,
                folder_child_id,
            );
        }

        builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", realm_id);
    });

    let realm_id = init_realm_id.unwrap();

    let alice = env.local_device("alice@dev1");

    let storage = WorkspaceDataStorage::start(&env.discriminant_dir, alice.clone(), realm_id)
        .await
        .unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    let expected_remote = [realm_id, folder_child_id, file_child_id];
    let expected_local = if with_local_changes {
        vec![realm_id, folder_child_id, file_child_id]
    } else {
        vec![]
    };
    assert_need_sync(need_sync, &expected_local, &expected_remote);
}

#[parsec_test(testbed = "minimal")]
async fn get_need_sync_entries_workspace(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let workspace_id = VlobID::from_hex("aa0000000000000000000000000000ff").unwrap();
    let child_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();

    let storage = WorkspaceDataStorage::start(&env.discriminant_dir, alice.clone(), workspace_id)
        .await
        .unwrap();

    // 1) Get from pristine storage, workspace manifest is a placeholder and hence must be synced

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[workspace_id], &[]);

    // 2) Pretent workspace manifest have been synced

    let (updater, mut manifest) = storage.for_update_workspace_manifest().await;
    {
        let manifest = Arc::make_mut(&mut manifest);
        manifest.base.version = 1;
        manifest.need_sync = false;
    }
    updater.update_workspace_manifest(manifest).await.unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[], &[]);

    // 3) Pretent remote change arrived

    // Child is not currently part of the storage and hence should be ignored
    storage
        .update_realm_checkpoint(2, vec![(workspace_id, 2), (child_id, 2)])
        .await
        .unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[], &[workspace_id]);

    // 4) Update storage to contains workspace v1 but with changes

    let (updater, mut manifest) = storage.for_update_workspace_manifest().await;
    {
        let manifest = Arc::make_mut(&mut manifest);
        manifest.need_sync = true;
    }
    updater.update_workspace_manifest(manifest).await.unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[workspace_id], &[workspace_id]);

    // 5) Update storage to contains workspace v2

    let (updater, mut manifest) = storage.for_update_workspace_manifest().await;
    {
        let manifest = Arc::make_mut(&mut manifest);
        manifest.base.version = 2;
        manifest.need_sync = false;
    }
    updater.update_workspace_manifest(manifest).await.unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[], &[]);
}

#[parsec_test(testbed = "minimal")]
#[case::no_update_delayed_flush(false)]
#[case::update_delayed_flush(true)]
async fn get_need_sync_entries_child(#[case] update_delayed_flush: bool, env: &TestbedEnv) {
    let mut init_realm_id = None;
    let env = env.customize(|builder| {
        let realm_id = builder.new_realm("alice").map(|e| e.realm_id);
        init_realm_id = Some(realm_id);

        // We don't care about workspace manifest here, so make it synchronized
        builder.create_or_update_workspace_manifest_vlob("alice@dev1", realm_id);
        builder.workspace_data_storage_fetch_workspace_vlob("alice@dev1", realm_id, None);
    });
    let realm_id = init_realm_id.unwrap();

    let alice = env.local_device("alice@dev1");
    let child1_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let child2_id = VlobID::from_hex("aa0000000000000000000000000000dd").unwrap();

    let storage = WorkspaceDataStorage::start(&env.discriminant_dir, alice.clone(), realm_id)
        .await
        .unwrap();

    // 1) Pretent remote change arrived, should be ignored since child is not present yet

    storage
        .update_realm_checkpoint(2, vec![(child1_id, 2)])
        .await
        .unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[], &[]);

    // 2) Create child placeholder

    let (updater, manifest) = storage.for_update_child_manifest(child2_id).await.unwrap();
    assert!(manifest.is_none()); // Child doesn't exist yet
    let manifest = {
        let timestamp = alice.now();
        Arc::new(LocalFileManifest {
            base: FileManifest {
                author: alice.device_id.clone(),
                timestamp,
                id: child2_id,
                parent: realm_id,
                version: 0,
                created: timestamp,
                updated: timestamp,
                blocksize: 512.try_into().unwrap(),
                size: 0,
                blocks: vec![],
            },
            blocksize: 512.try_into().unwrap(),
            size: 0,
            blocks: vec![],
            need_sync: true,
            updated: timestamp,
        })
    };
    updater
        .update_as_file_manifest(manifest, update_delayed_flush, [].into_iter())
        .await
        .unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[child2_id], &[]);

    // 3) Sync child

    let (updater, manifest) = storage.for_update_child_manifest(child2_id).await.unwrap();
    let manifest = {
        match manifest.expect("child present in local storage") {
            ArcLocalChildManifest::File(mut manifest) => {
                let m = Arc::make_mut(&mut manifest);
                m.base.version = 1;
                m.need_sync = false;
                manifest
            }
            _ => unreachable!(),
        }
    };
    updater
        .update_as_file_manifest(manifest, update_delayed_flush, [].into_iter())
        .await
        .unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[], &[]);

    // 4) Pretent remote change arrived for child

    storage
        .update_realm_checkpoint(3, vec![(child1_id, 3), (child2_id, 2)])
        .await
        .unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[], &[child2_id]);

    // 5) Child got local changes

    let (updater, manifest) = storage.for_update_child_manifest(child2_id).await.unwrap();
    let manifest = {
        match manifest.expect("child present in local storage") {
            ArcLocalChildManifest::File(mut manifest) => {
                let m = Arc::make_mut(&mut manifest);
                m.need_sync = true;
                manifest
            }
            _ => unreachable!(),
        }
    };
    updater
        .update_as_file_manifest(manifest, update_delayed_flush, [].into_iter())
        .await
        .unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[child2_id], &[child2_id]);

    // 6) Child synced both remote and local changes

    let (updater, manifest) = storage.for_update_child_manifest(child2_id).await.unwrap();
    let manifest = {
        match manifest.expect("child present in local storage") {
            ArcLocalChildManifest::File(mut manifest) => {
                let m = Arc::make_mut(&mut manifest);
                m.need_sync = false;
                m.base.version = 2;
                manifest
            }
            _ => unreachable!(),
        }
    };
    updater
        .update_as_file_manifest(manifest, update_delayed_flush, [].into_iter())
        .await
        .unwrap();

    let need_sync = storage.get_need_sync_entries().await.unwrap();
    assert_need_sync(need_sync, &[], &[]);
}

#[parsec_test(testbed = "minimal")]
async fn workspace_manifest(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ff").unwrap();

    let storage = WorkspaceDataStorage::start(&env.discriminant_dir, alice.clone(), realm_id)
        .await
        .unwrap();

    // See unit tests for bad start

    // 1) Get speculative workspace manifest

    let workspace_manifest = storage.get_workspace_manifest();
    let expected = LocalWorkspaceManifest {
        base: WorkspaceManifest {
            author: alice.device_id.clone(),
            timestamp: workspace_manifest.updated,
            id: realm_id,
            version: 0,
            created: workspace_manifest.updated,
            updated: workspace_manifest.updated,
            children: HashMap::new(),
        },
        need_sync: true,
        updated: workspace_manifest.updated,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: true,
    };
    p_assert_eq!(*workspace_manifest, expected);

    // 2) Update workspace manifest

    let (updater, mut manifest) = storage.for_update_workspace_manifest().await;
    {
        let m = Arc::make_mut(&mut manifest);
        m.base.version = 1;
        m.speculative = false;
        m.need_sync = false;
    }
    let expected = manifest.clone();
    updater.update_workspace_manifest(manifest).await.unwrap();

    // 3) Get back workspace manifest

    let manifest = storage.get_workspace_manifest();
    p_assert_eq!(manifest, expected);

    // 4) Get back workspace manifest from restarted storage

    storage.stop().await.unwrap();

    let storage = WorkspaceDataStorage::start(&env.discriminant_dir, alice.clone(), realm_id)
        .await
        .unwrap();

    let manifest = storage.get_workspace_manifest();
    p_assert_eq!(manifest, expected);
}

#[parsec_test(testbed = "minimal")]
async fn non_speculative_init(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ff").unwrap();

    // 1) Initialize the database

    workspace_storage_non_speculative_init(&env.discriminant_dir, &alice, realm_id)
        .await
        .unwrap();

    // 2) Check the database content

    let storage = WorkspaceDataStorage::start(&env.discriminant_dir, alice.clone(), realm_id)
        .await
        .unwrap();

    let workspace_manifest = storage.get_workspace_manifest();
    let expected = LocalWorkspaceManifest {
        base: WorkspaceManifest {
            author: alice.device_id.clone(),
            timestamp: workspace_manifest.updated,
            id: realm_id,
            version: 0,
            created: workspace_manifest.updated,
            updated: workspace_manifest.updated,
            children: HashMap::new(),
        },
        need_sync: true,
        updated: workspace_manifest.updated,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };
    p_assert_eq!(*workspace_manifest, expected);
}

// TODO: test that flush_work_ahead_of_db doesn't lose data if it fails
