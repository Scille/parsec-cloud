// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// This module provide helpers that are used for testing purpose.
// To simplify the writing of those helpers, we use the same rule for when writing tests.
#![allow(clippy::unwrap_used)]

use std::{any::Any, ops::Deref, path::Path, sync::Arc};

use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_testbed::{
    test_get_testbed, test_get_testbed_component_store, TestbedEnv, TestbedEvent,
};
use libparsec_types::prelude::*;

use crate::{
    certificates::CertificatesStorage,
    user::UserStorage,
    workspace::{WorkspaceCacheStorage, WorkspaceDataStorage},
};

const STORE_ENTRY_KEY: &str = "platform_storage";

enum StorageKind {
    Certificates,
    #[allow(dead_code)]
    User,
    #[allow(dead_code)]
    WorkspaceData(VlobID),
    #[allow(dead_code)]
    WorkspaceCache(VlobID),
}

struct ComponentStore {
    populated: AsyncMutex<Vec<(DeviceID, StorageKind)>>,
}

fn store_factory(_env: &TestbedEnv) -> Arc<dyn Any + Send + Sync> {
    Arc::new(ComponentStore {
        populated: AsyncMutex::new(vec![]),
    })
}

#[allow(unused)]
pub(crate) async fn maybe_populate_certificate_storage(data_base_dir: &Path, device: &LocalDevice) {
    let disabled = std::env::var("TESTBED_DISABLE_POPULATE_CERTIFICATE_STORAGE").is_ok();
    if disabled {
        return;
    }

    if let Some(store) = test_get_testbed_component_store::<ComponentStore>(
        data_base_dir,
        STORE_ENTRY_KEY,
        store_factory,
    ) {
        let mut guard = store.populated.lock().await;
        let already_populated = guard.iter().any(|(candidate, kind)| {
            matches!(kind, StorageKind::Certificates) && *candidate == device.device_id
        });

        if !already_populated {
            let env = test_get_testbed(data_base_dir).expect("Testbed existence already checked");

            // 1) Do we need to be initialized ? and by fetching up what certificate index ?
            let up_to = env.template.events.iter().rev().find_map(|e| match e {
                TestbedEvent::CertificatesStorageFetchCertificates(x)
                    if x.device == device.device_id =>
                {
                    Some(x.up_to)
                }
                _ => None,
            });

            // 2) Actually do the initialization
            if let Some(up_to) = up_to {
                let need_redacted = matches!(
                    env.template.user_profile_at(device.user_id(), up_to),
                    UserProfile::Outsider
                );

                let mut storage = CertificatesStorage::no_populate_start(data_base_dir, device)
                    .await
                    .unwrap();
                let mut update = storage.for_update().await.unwrap();

                let certifs = env.template.certificates().take_while(|c| {
                    let certif_timestamp = match &c.certificate {
                        AnyArcCertificate::User(c) => c.timestamp,
                        AnyArcCertificate::Device(c) => c.timestamp,
                        AnyArcCertificate::UserUpdate(c) => c.timestamp,
                        AnyArcCertificate::RevokedUser(c) => c.timestamp,
                        AnyArcCertificate::RealmRole(c) => c.timestamp,
                        AnyArcCertificate::RealmName(c) => c.timestamp,
                        AnyArcCertificate::RealmArchiving(c) => c.timestamp,
                        AnyArcCertificate::RealmKeyRotation(c) => c.timestamp,
                        AnyArcCertificate::ShamirRecoveryBrief(c) => c.timestamp,
                        AnyArcCertificate::ShamirRecoveryShare(c) => c.timestamp,
                        AnyArcCertificate::SequesterAuthority(c) => c.timestamp,
                        AnyArcCertificate::SequesterService(c) => c.timestamp,
                        AnyArcCertificate::SequesterRevokedService(c) => c.timestamp,
                    };
                    certif_timestamp <= up_to
                });
                for (offset, certif) in certifs.enumerate() {
                    let signed = if need_redacted {
                        &certif.signed_redacted
                    } else {
                        &certif.signed
                    };
                    let encrypted = device.local_symkey.encrypt(signed);
                    match &certif.certificate {
                        AnyArcCertificate::User(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::Device(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::UserUpdate(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::RevokedUser(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::RealmRole(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::RealmName(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::RealmKeyRotation(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::RealmArchiving(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::ShamirRecoveryBrief(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::ShamirRecoveryShare(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::SequesterAuthority(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::SequesterService(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                        AnyArcCertificate::SequesterRevokedService(certif) => update
                            .add_certificate(certif.deref(), encrypted)
                            .await
                            .unwrap(),
                    }
                }
                update.commit().await.unwrap();

                storage.stop().await;
            }

            // Mark as populated
            guard.push((device.device_id.clone(), StorageKind::Certificates));
        }
    }
}

#[allow(unused)]
pub(crate) async fn maybe_populate_user_storage(data_base_dir: &Path, device: &LocalDevice) {
    if let Some(store) = test_get_testbed_component_store::<ComponentStore>(
        data_base_dir,
        STORE_ENTRY_KEY,
        store_factory,
    ) {
        let mut guard = store.populated.lock().await;
        let already_populated = guard.iter().any(|(candidate, kind)| {
            matches!(kind, StorageKind::User) && *candidate == device.device_id
        });

        if !already_populated {
            let env = test_get_testbed(data_base_dir).expect("Testbed existence already checked");

            // Only start the storage if we need to do some initialization work
            let mut lazy_storage: Option<UserStorage> = None;

            for event in &env.template.events {
                let (maybe_manifest, maybe_checkpoint) = match event {
                    TestbedEvent::UserStorageFetchUserVlob(x) if x.device == device.device_id => {
                        (Some(x.local_manifest.clone()), None)
                    }
                    TestbedEvent::UserStorageLocalUpdate(x) if x.device == device.device_id => {
                        (Some(x.local_manifest.clone()), None)
                    }
                    TestbedEvent::UserStorageFetchRealmCheckpoint(x)
                        if x.device == device.device_id =>
                    {
                        (None, Some((x.checkpoint, x.remote_user_manifest_version)))
                    }
                    _ => continue,
                };

                if lazy_storage.is_none() {
                    lazy_storage = Some(
                        UserStorage::no_populate_start(data_base_dir, device)
                            .await
                            .unwrap(),
                    );
                }

                if let Some((checkpoint, remote_user_manifest_version)) = maybe_checkpoint {
                    lazy_storage
                        .as_mut()
                        .unwrap()
                        .update_realm_checkpoint(checkpoint, remote_user_manifest_version)
                        .await
                        .unwrap();
                }

                if let Some(manifest) = maybe_manifest {
                    let encrypted = manifest.dump_and_encrypt(&device.local_symkey);
                    lazy_storage
                        .as_mut()
                        .unwrap()
                        .update_user_manifest(&encrypted, manifest.need_sync, manifest.base.version)
                        .await
                        .unwrap();
                }
            }

            if let Some(storage) = lazy_storage {
                storage.stop().await;
            }

            // Mark as populated
            guard.push((device.device_id.clone(), StorageKind::User));
        }
    }
}

#[allow(unused)]
pub(crate) async fn maybe_populate_workspace_data_storage(
    data_base_dir: &Path,
    device: Arc<LocalDevice>,
    realm_id: VlobID,
) {
    if let Some(store) = test_get_testbed_component_store::<ComponentStore>(
        data_base_dir,
        STORE_ENTRY_KEY,
        store_factory,
    ) {
        let mut guard = store.populated.lock().await;
        let already_populated = guard.iter().any(|(candidate, kind)| {
            matches!(kind, StorageKind::WorkspaceData(candidate_realm_id) if *candidate_realm_id == realm_id) && *candidate == device.device_id
        });

        if !already_populated {
            let env = test_get_testbed(data_base_dir).expect("Testbed existence already checked");

            let mut lazy_storage: Option<WorkspaceDataStorage> = None;

            for event in &env.template.events {
                let (
                    maybe_workspace_manifest,
                    maybe_folder_manifest,
                    maybe_file_manifest,
                    maybe_checkpoint,
                ) = match event {
                    TestbedEvent::WorkspaceDataStorageFetchWorkspaceVlob(x)
                        if x.device == device.device_id && x.realm == realm_id =>
                    {
                        (Some(x.local_manifest.clone()), None, None, None)
                    }
                    TestbedEvent::WorkspaceDataStorageFetchFolderVlob(x)
                        if x.device == device.device_id && x.realm == realm_id =>
                    {
                        (None, Some(x.local_manifest.clone()), None, None)
                    }
                    TestbedEvent::WorkspaceDataStorageFetchFileVlob(x)
                        if x.device == device.device_id && x.realm == realm_id =>
                    {
                        (None, None, Some(x.local_manifest.clone()), None)
                    }
                    TestbedEvent::WorkspaceDataStorageLocalWorkspaceManifestUpdate(x)
                        if x.device == device.device_id && x.realm == realm_id =>
                    {
                        (Some(x.local_manifest.clone()), None, None, None)
                    }
                    TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(x)
                        if x.device == device.device_id && x.realm == realm_id =>
                    {
                        (None, Some(x.local_manifest.clone()), None, None)
                    }
                    TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(x)
                        if x.device == device.device_id && x.realm == realm_id =>
                    {
                        (None, None, Some(x.local_manifest.clone()), None)
                    }
                    TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(x)
                        if x.device == device.device_id && x.realm == realm_id =>
                    {
                        (
                            None,
                            None,
                            None,
                            Some((x.checkpoint, x.changed_vlobs.clone())),
                        )
                    }
                    _ => continue,
                };

                if lazy_storage.is_none() {
                    lazy_storage = Some(
                        WorkspaceDataStorage::no_populate_start(
                            data_base_dir,
                            device.clone(),
                            realm_id,
                        )
                        .await
                        .unwrap(),
                    );
                }

                if let Some((checkpoint, changed_vlobs)) = maybe_checkpoint {
                    lazy_storage
                        .as_ref()
                        .unwrap()
                        .update_realm_checkpoint(checkpoint, changed_vlobs)
                        .await
                        .unwrap();
                }

                if let Some(manifest) = maybe_workspace_manifest {
                    lazy_storage
                        .as_ref()
                        .unwrap()
                        .for_update_workspace_manifest()
                        .await
                        .0
                        .update_workspace_manifest(manifest)
                        .await
                        .unwrap();
                }

                if let Some(manifest) = maybe_folder_manifest {
                    lazy_storage
                        .as_ref()
                        .unwrap()
                        .for_update_child_manifest(manifest.base.id)
                        .await
                        .unwrap()
                        .0
                        .update_as_folder_manifest(manifest)
                        .await
                        .unwrap();
                }

                if let Some(manifest) = maybe_file_manifest {
                    lazy_storage
                        .as_ref()
                        .unwrap()
                        .for_update_child_manifest(manifest.base.id)
                        .await
                        .unwrap()
                        .0
                        .update_as_file_manifest(manifest, false, [].into_iter())
                        .await
                        .unwrap();
                }
            }

            if let Some(storage) = lazy_storage {
                storage.stop().await;
            }

            // Mark as populated
            guard.push((
                device.device_id.clone(),
                StorageKind::WorkspaceData(realm_id),
            ));
        }
    }
}

#[allow(unused)]
pub(crate) async fn maybe_populate_workspace_cache_storage(
    data_base_dir: &Path,
    device: Arc<LocalDevice>,
    realm_id: VlobID,
) {
    if let Some(store) = test_get_testbed_component_store::<ComponentStore>(
        data_base_dir,
        STORE_ENTRY_KEY,
        store_factory,
    ) {
        let mut guard = store.populated.lock().await;
        let already_populated = guard.iter().any(|(candidate, kind)| {
            matches!(kind, StorageKind::WorkspaceCache(candidate_realm_id) if *candidate_realm_id == realm_id) && *candidate == device.device_id
        });

        if !already_populated {
            let env = test_get_testbed(data_base_dir).expect("Testbed existence already checked");

            let mut lazy_storage: Option<WorkspaceCacheStorage> = None;

            for event in &env.template.events {
                let (block_id, cleartext_block) = match event {
                    TestbedEvent::WorkspaceCacheStorageFetchBlock(x)
                        if x.device == device.device_id && x.realm == realm_id =>
                    {
                        (x.block_id, &x.cleartext)
                    }
                    _ => continue,
                };

                if lazy_storage.is_none() {
                    lazy_storage = Some(
                        // Set cache_size to max to disable any garbage collection
                        WorkspaceCacheStorage::no_populate_start(
                            data_base_dir,
                            u64::MAX,
                            device.clone(),
                            realm_id,
                        )
                        .await
                        .unwrap(),
                    );
                }

                lazy_storage
                    .as_ref()
                    .unwrap()
                    .set_block(block_id, cleartext_block)
                    .await
                    .unwrap();
            }

            if let Some(storage) = lazy_storage {
                storage.stop().await;
            }

            // Mark as populated
            guard.push((
                device.device_id.clone(),
                StorageKind::WorkspaceCache(realm_id),
            ));
        }
    }
}
