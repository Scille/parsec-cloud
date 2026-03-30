// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

/// WorkspaceArchived contains:
/// - 2 users: `alice` (admin), `bob` (regular)
/// - Bob is revoked
/// - devices `alice@dev1` whose storages is up to date
/// - workspace `wksp_archived` shared to alice (owner) and marked as archived
/// - workspace `wksp_soon_to_delete` shared to alice (owner) and marked as deletion planned (with deletion date not yet reached)
/// - workspace `wksp_ready_to_delete` shared to alice (owner) and marked as deletion planned (with deletion date reached)
/// - workspace `wksp_deleted` shared to alice (owner) that has already been deleted
/// - workspace `wksp_not_longer_to_delete` shared to alice (owner) that was marked as deletion planned in the past
/// - workspace `wksp_orphaned` shared to bob (owner) before his revocation
/// - workspace `wksp_orphaned_and_ready_to_delete` shared to bob (owner) before his revocation and marked as deletion
///   planned (with deletion date reached)
/// - workspace `wksp_archived` contains a file `/file.txt` with content b"hello"
/// - workspace `wksp_ready_to_delete` also contains 2 vlobs and 5 blocks
/// - other workspaces don't contain any vlob&block
/// - All those workspaces and bootstrapped
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    // If you change something here:
    // - Update this function's docstring
    // - Update `server/tests/common/client.py::WorkspaceArchivedOrgRpcClients`s docstring

    let mut builder = TestbedTemplate::from_builder("workspace_archived");

    // 1) Create user & devices

    builder.bootstrap_organization("alice"); // alice@dev1
    builder
        .new_user("bob")
        .with_initial_profile(UserProfile::Standard); // bob@dev1

    // 2) Create workspaces

    let wksp_archived_id = builder.new_realm("alice").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp_archived_id);
    let wksp_archived_name = builder
        .rename_realm(wksp_archived_id, "wksp_archived")
        .map(|e| e.name.clone());
    let wksp_archived_block_data = b"hello";
    let wksp_archived_block_id = builder
        .create_block(
            "alice@dev1",
            wksp_archived_id,
            Bytes::from_static(wksp_archived_block_data),
        )
        .map(|e| e.block_id);
    let wksp_archived_file_id = builder
        .create_or_update_file_manifest_vlob("alice@dev1", wksp_archived_id, None, wksp_archived_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.size = wksp_archived_block_data.len() as u64;
            manifest.blocks.push(BlockAccess {
                id: wksp_archived_block_id,
                offset: 0,
                size: (wksp_archived_block_data.len() as u64).try_into().unwrap(),
                digest: HashDigest::from_data(wksp_archived_block_data),
            });
        })
        .map(|e| e.manifest.id);
    builder
        .create_or_update_workspace_manifest_vlob("alice@dev1", wksp_archived_id)
        .customize_children([("file.txt", Some(wksp_archived_file_id))].into_iter());

    builder.archive_realm(wksp_archived_id, RealmArchivingConfiguration::Archived);
    builder.store_stuff("wksp_archived_id", &wksp_archived_id);
    builder.store_stuff("wksp_archived_name", &wksp_archived_name);

    let wksp_soon_to_delete_id = builder.new_realm("alice").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp_soon_to_delete_id);
    let wksp_soon_to_delete_name = builder
        .rename_realm(wksp_soon_to_delete_id, "wksp_soon_to_delete")
        .map(|e| e.name.clone());
    builder.archive_realm(
        wksp_soon_to_delete_id,
        RealmArchivingConfiguration::DeletionPlanned {
            // To delete far away in the future...
            deletion_date: "2999-01-01T00:00:00Z".parse().unwrap(),
        },
    );
    builder.store_stuff("wksp_soon_to_delete_id", &wksp_soon_to_delete_id);
    builder.store_stuff("wksp_soon_to_delete_name", &wksp_soon_to_delete_name);

    let wksp_ready_to_delete_id = builder.new_realm("alice").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp_ready_to_delete_id);
    let wksp_ready_to_delete_name = builder
        .rename_realm(wksp_ready_to_delete_id, "wksp_ready_to_delete")
        .map(|e| e.name.clone());
    builder.create_or_update_workspace_manifest_vlob("alice@dev1", wksp_ready_to_delete_id);
    builder.create_or_update_file_manifest_vlob(
        "alice@dev1",
        wksp_ready_to_delete_id,
        None,
        wksp_ready_to_delete_id,
    );
    for _ in 0..5 {
        builder.create_block(
            "alice@dev1",
            wksp_ready_to_delete_id,
            Bytes::from_static(b""),
        );
    }
    builder.archive_realm(
        wksp_ready_to_delete_id,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: "2010-01-01T00:00:00Z".parse().unwrap(),
        },
    );
    builder.store_stuff("wksp_ready_to_delete_id", &wksp_ready_to_delete_id);
    builder.store_stuff("wksp_ready_to_delete_name", &wksp_ready_to_delete_name);

    let wksp_deleted_id = builder.new_realm("alice").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp_deleted_id);
    let wksp_deleted_name = builder
        .rename_realm(wksp_deleted_id, "wksp_deleted")
        .map(|e| e.name.clone());
    builder.archive_realm(
        wksp_deleted_id,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: "2010-01-01T00:00:00Z".parse().unwrap(),
        },
    );
    builder
        .delete_realm(wksp_deleted_id)
        .customize(|e| e.timestamp = "2010-01-02T00:00:00Z".parse().unwrap());
    builder.store_stuff("wksp_deleted_id", &wksp_deleted_id);
    builder.store_stuff("wksp_deleted_name", &wksp_deleted_name);

    let wksp_not_longer_to_delete_id = builder.new_realm("alice").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp_not_longer_to_delete_id);
    let wksp_not_longer_to_delete_name = builder
        .rename_realm(wksp_not_longer_to_delete_id, "wksp_not_longer_to_delete")
        .map(|e| e.name.clone());
    builder.archive_realm(
        wksp_not_longer_to_delete_id,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: "2010-01-01T00:00:00Z".parse().unwrap(),
        },
    );
    builder.archive_realm(
        wksp_not_longer_to_delete_id,
        RealmArchivingConfiguration::Available,
    );
    builder.store_stuff(
        "wksp_not_longer_to_delete_id",
        &wksp_not_longer_to_delete_id,
    );
    builder.store_stuff(
        "wksp_not_longer_to_delete_name",
        &wksp_not_longer_to_delete_name,
    );

    let wksp_orphaned_id = builder.new_realm("bob").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp_orphaned_id);
    let wksp_orphaned_name = builder
        .rename_realm(wksp_orphaned_id, "wksp_orphaned")
        .map(|e| e.name.clone());
    builder.store_stuff("wksp_orphaned_id", &wksp_orphaned_id);
    builder.store_stuff("wksp_orphaned_name", &wksp_orphaned_name);

    let wksp_orphaned_and_ready_to_delete_id = builder.new_realm("bob").map(|e| e.realm_id);
    builder.rotate_key_realm(wksp_orphaned_and_ready_to_delete_id);
    let wksp_orphaned_and_ready_to_delete_name = builder
        .rename_realm(
            wksp_orphaned_and_ready_to_delete_id,
            "wksp_orphaned_and_ready_to_delete",
        )
        .map(|e| e.name.clone());
    builder.archive_realm(
        wksp_orphaned_and_ready_to_delete_id,
        RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: "2010-01-01T00:00:00Z".parse().unwrap(),
        },
    );
    builder.store_stuff(
        "wksp_orphaned_and_ready_to_delete_id",
        &wksp_orphaned_and_ready_to_delete_id,
    );
    builder.store_stuff(
        "wksp_orphaned_and_ready_to_delete_name",
        &wksp_orphaned_and_ready_to_delete_name,
    );

    // 3) Revoke Bob

    builder.revoke_user("bob");

    // 5) Initialize client storages for alice@dev1

    builder.certificates_storage_fetch_certificates("alice@dev1");
    builder
        .user_storage_local_update("alice@dev1")
        .update_local_workspaces_with_fetched_certificates();
    builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", wksp_archived_id);
    builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", wksp_soon_to_delete_id);
    builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", wksp_ready_to_delete_id);
    builder
        .workspace_data_storage_fetch_realm_checkpoint("alice@dev1", wksp_not_longer_to_delete_id);

    builder.finalize()
}
